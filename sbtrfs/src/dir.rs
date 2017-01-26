use std::io::{Write, Read, Cursor};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct Dir {
    files: Vec<DirEntry>
}

pub struct DirEntry(u32, String);

impl DirEntry {
    pub fn serialise( &self ) -> Vec<u8> {
       let mut buf: Vec<u8> = Vec::new();
       buf.write_u32::<BigEndian>( self.0 );
       let mut text = self.1.clone();
       text.truncate(252);
       buf.write_all( text.as_bytes() );
       return buf;
    }

    pub fn deserialise( buf: &[u8] ) -> DirEntry {
        let mut buf = Cursor::new(buf.to_vec());
        let int: u32 = buf.read_u32::<BigEndian>().unwrap();
        let mut strvec: Vec<u8> = Vec::new();
        buf.read_to_end( &mut strvec );
        let string = String::from_utf8( strvec ).unwrap();
        DirEntry( int, string )
    }
}
