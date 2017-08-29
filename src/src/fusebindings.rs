use std::path::Path;
use std::io::{Write, Read, Cursor, SeekFrom, Seek};
use libc::{ENOENT, ENOSYS};
use time::{self,Timespec};
use fuse::{self, FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
use std::fs::{File, OpenOptions};
use super::inode::INode;
use super::bitmap::BitMap;
use super::dir::Dir;
use std::ffi::OsStr;
use crypto::aes::{self, KeySize};
use crypto::symmetriccipher::SynchronousStreamCipher;


pub struct FuseBindFS {
    medium: File,
    pub dmap: BitMap,
    pub imap: BitMap
}

impl FuseBindFS {
    pub fn new( mut file: File ) -> FuseBindFS {
        let mut dmapbuf: Vec<u8> = Vec::new();
        let mut imapbuf: Vec<u8> = Vec::new();
        dmapbuf.resize(4096, 0);
        imapbuf.resize(4096, 0);

        file.seek(SeekFrom::Start( 4096 ));
        file.read_exact( &mut dmapbuf );
        file.read_exact( &mut imapbuf );

        FuseBindFS {
            medium: file,
            dmap: BitMap::deserialise( &dmapbuf, 4096, 4096 ),
            imap: BitMap::deserialise( &imapbuf, 8192, 4096 )
        }
    }

    pub fn make_dir( &mut self, uid: u32, gid: u32, mode: u16, parent: u32 ) -> u32 {
        let pointer = self.dmap.next_free(1).unwrap();
        self.dmap.set_bit( pointer, false );
        let inonum = self.imap.next_free(1).unwrap();
        self.imap.set_bit( inonum, false );
        let inode = INode::new( uid, gid, 0, pointer, mode, 10 , parent);
        self.write_inode( &inode, inonum );
        return inonum;
    }

    pub fn read_data ( &mut self, pointer: u32 ) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0;4096];
        self.medium.seek( SeekFrom::Start( 20000 + 4096*pointer as u64 ));
        self.medium.read_exact( &mut buf );
        return buf;
    }

    pub fn write_data( &mut self, pointer: u32, buf: &[u8] ) {
        self.medium.seek( SeekFrom::Start( 20000 + 4096*pointer as u64 ));
        self.medium.write_all( buf );
    }


    pub fn read_dir ( &mut self, inonum: u32 ) -> Dir {
        let inode = self.read_inode( inonum );
        let mut dir = Dir::deserialise( &self.read_data( inode.pointer ));
        dir.inode = inode.pointer;
        dir.parent_inode = inode.parent;
        dir
    }


    fn write_inode( &mut self, inode: &INode, inonum: u32 ) {
        self.medium.seek( SeekFrom::Start( 12288 + 256*inonum as u64 ));
        self.medium.write( &inode.serialise() );
    }

    fn encrypt_data( &mut data: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
        let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
        let mut output: Vec<u8> = repeat(0u8).take(data.len()).collect();
        cipher.process(secret.as_bytes(), &mut output[..]);
    }


    fn read_inode( &mut self, inonum: u32 ) -> INode {
        let mut inodebuf: Vec<u8> = vec![0; 256];
        self.medium.seek(SeekFrom::Start( 12288 + 256*inonum as u64 ));
        self.medium.read_exact( &mut inodebuf );
        INode::deserialise( &inodebuf )
    }
}

impl Filesystem for FuseBindFS {
    fn mkdir( &mut self, _req: &Request, _parent: u64, _name: &OsStr, _mode: u32, reply: ReplyEntry ) {
        self.make_dir( _req.uid(), _req.gid(), _mode as u16, _parent as u32 );
    }

    fn lookup( &mut self, _req: &Request, parent:u64, name:&OsStr, reply: ReplyEntry) {
        println!("lookup(parent={}, name={})", parent, name.to_str().unwrap());
        reply.error(ENOENT);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        let ts = time::get_time();
        let inode = self.read_inode( ino as u32 );
        let ftype = if inode.stype > 5 { FileType::Directory} else { FileType::RegularFile };

        let attr = FileAttr {
            ino: ino, size: inode.size as u64, blocks: 1, atime: ts, mtime: ts,
            ctime: ts, crtime:ts, kind: ftype,
            perm: inode.mode, nlink: 0, uid: inode.uid, gid: inode.gid, rdev: 0, flags: 0
        };

        let ttl = Timespec::new(1,0);
        let res = !self.imap.read_bit( ino as u32);
        if res {
            reply.attr( &ttl, &attr );
        } else {
            reply.error( ENOENT );
        }
    }

    fn readdir( &mut self, _req: &Request, ino: u64, fh: u64,
                offset: u64, mut reply: ReplyDirectory) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        if self.imap.read_bit( ino as u32 ) == true 
        {
            reply.error( ENOENT );
            return;
        }
        let dir = self.read_dir( ino as u32 );
        reply.add( dir.inode as u64, self.read_inode( dir.inode ).pointer as u64, FileType::Directory, &Path::new("."));
        reply.add( dir.parent_inode as u64, self.read_inode(dir.parent_inode).pointer as u64, FileType::Directory, &Path::new(".."));

        for file in dir.files {
            let inode = self.read_inode(file.0);
            let ftype = if inode.stype > 5 { FileType::Directory } else { FileType::RegularFile };
            reply.add( file.0 as u64, inode.pointer as u64,  ftype, &Path::new( &file.1 ));
        }
        reply.ok();
    }

}

pub fn init_mount( volume: &Path, mountpoint: &Path ) {
    
    let mut file = match  OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(true)
                           .open( volume  ) {
        Ok(file) => file,
        Err(_) => panic!("File at path {:?} can't be accessed", volume )
                           };
    
    let fs = FuseBindFS::new( file );

    fuse::mount(fs, &mountpoint, &[]);
}
