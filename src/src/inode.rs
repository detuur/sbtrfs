use std::io::{Write, Read, Cursor};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub struct INode {
    pub uid: u32,
    pub gid: u32,
    pub size: u32,
    pub pointer: u32,
    pub mode: u16,
    pub stype: u8,
    pub parent: u32,
    padding: [u8; 233]
}

impl INode {
    pub fn serialise( &self ) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_u32::<BigEndian>( self.uid );
        buf.write_u32::<BigEndian>( self.gid );
        buf.write_u32::<BigEndian>( self.size );
        buf.write_u32::<BigEndian>( self.pointer );
        buf.write_u16::<BigEndian>( self.mode );
        buf.write_u8( self.stype );
        buf.write_u32::<BigEndian>( self.parent );
        assert_eq!( buf.len() + self.padding.len(), 256);
        return buf;
    }

    pub fn deserialise( buf: &[u8] ) -> INode {
        let mut rdr = Cursor::new(buf);
        INode {
            uid             : rdr.read_u32::<BigEndian>().unwrap(),
            gid             : rdr.read_u32::<BigEndian>().unwrap(),
            size            : rdr.read_u32::<BigEndian>().unwrap(),
            pointer         : rdr.read_u32::<BigEndian>().unwrap(),
            mode            : rdr.read_u16::<BigEndian>().unwrap(),
            stype           : rdr.read_u8().unwrap(),
            parent          : rdr.read_u32::<BigEndian>().unwrap(),
            padding         : [0; 233]
        }
    }

    pub fn new( uid: u32, gid: u32, size: u32, pointer: u32, mode: u16, stype:u8, parent: u32 ) -> INode {
        INode { uid:uid, gid:gid, size:size, pointer:pointer, mode:mode, stype:stype, parent:parent, padding: [0;233] }
    }
            

    pub fn num_of_blocks( &self ) -> u32 {
        return self.size / 4096 + 1;
    }
}
