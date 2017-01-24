use std::io::Write;
use byteorder::{BigEndian, WriteBytesExt};


struct INode {
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

    pub fn num_of_blocks( &self ) -> u32 {
        return self.size / 4096 + 1;
    }
}
