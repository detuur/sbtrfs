use std::io::{Write, Read, Cursor};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub struct INode {
    permissions: u16,
    uid: u32,
    size: u32,
    pointer: u32,
    mode: u8,
    padding: [u8; 241]
}

impl INode {
    pub fn serialise( &self ) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_u16::<BigEndian>( self.permissions );
        buf.write_u32::<BigEndian>( self.uid );
        buf.write_u32::<BigEndian>( self.size );
        buf.write_u32::<BigEndian>( self.pointer );
        buf.write_u8( self.mode );
        assert_eq!( buf.len() + self.padding.len(), 256);
        return buf;
    }

    pub fn deserialise( buf: &[u8] ) -> INode {
        let mut rdr = Cursor::new(buf);
        INode {
            permissions     : rdr.read_u16::<BigEndian>().unwrap(),
            uid             : rdr.read_u32::<BigEndian>().unwrap(),
            size            : rdr.read_u32::<BigEndian>().unwrap(),
            pointer         : rdr.read_u32::<BigEndian>().unwrap(),
            mode            : rdr.read_u8().unwrap(),
            padding         : [0; 241]
        }
    }
            

    pub fn num_of_blocks( &self ) -> u32 {
        return self.size / 4096 + 1;
    }
}
