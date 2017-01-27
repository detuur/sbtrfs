use std::path::Path;
use std::io::{Write, Read, Cursor, SeekFrom, Seek};
use libc::{ENOENT, ENOSYS};
use time::Timespec;
use fuse::{self, FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
use std::fs::{File, OpenOptions};
use super::inode::INode;
use super::bitmap::BitMap;

struct FuseBindFS {
    medium: File,
    dmap: BitMap,
    imap: BitMap
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

}

impl Filesystem for FuseBindFS {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        let ts = Timespec::new(0,0);
        let attr = FileAttr {
            ino: 1, size: 0, blocks: 0, atime: ts, mtime: ts,
            ctime: ts, crtime:ts, kind: FileType::Directory,
            perm: 0o755, nlink: 0, uid: 0, gid: 0, rdev: 0, flags: 0
        };
        let ttl = Timespec::new(1,0);
        if ino == 1 {
            reply.attr( &ttl, &attr );
        } else {
            reply.error( ENOENT );
        }
    }

    fn readdir( &mut self, _req: &Request, ino: u64, fh: u64,
                offset: u64, mut reply: ReplyDirectory) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, &Path::new("."));
                reply.add(1, 1, FileType::Directory, &Path::new(".."));
            }
            reply.ok();
        } else {
            reply.error( ENOENT );
        }
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
