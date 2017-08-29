use std::io::{Write, Read, Cursor};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct Dir {
    pub inode: u32,
    pub parent_inode: u32,
    pub files: Vec<DirEntry>
}

#[derive(Clone)]
pub struct DirEntry( pub u32, pub String);

impl Dir {
    pub fn serialise( &self ) -> Vec<u8> {
       let mut buf: Vec<u8> = Vec::new();
       buf.write_u8( self.files.len() as u8 );
       for file in self.files.clone() {
           buf.write_u16::<BigEndian>( file.1.len() as u16 );  // write filename length
           buf.write_u32::<BigEndian>( file.0 );        // write inode
           buf.write_all( file.1.as_bytes() );          // write filename
       }
       return buf;
    }

    pub fn deserialise( buf: &[u8] ) -> Dir {
        let mut buf = Cursor::new(buf.to_vec());
        let mut files: Vec<DirEntry> = Vec::new();
        let fileslen = buf.read_u8().unwrap();
        for it in 0..fileslen {
            let fnlen = buf.read_u16::<BigEndian>().unwrap();
            let inode = buf.read_u32::<BigEndian>().unwrap();
            let mut strbuf = vec![0; fnlen as usize ];
            buf.read( &mut strbuf );
            let entry = DirEntry( inode, String::from_utf8( strbuf).unwrap() );
            files.push(entry);
        }
        Dir{ inode: u32::max_value(), parent_inode: u32::max_value(), files: files }
    }
}
