use std::io::{Write, Read, Cursor};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub struct BitMap {
    map: Vec<u8>,
    offset: u32,
    size: u32
}

impl BitMap {
    pub fn deserialise( buf: &[u8], offset: u32, size: u32 ) -> BitMap {
        let mut rdr = Cursor::new(buf);
        let mut newmap: Vec<u8> = Vec::new();
        rdr.read_to_end( &mut newmap );

        BitMap {
            size: size,
            offset: offset,
            map: newmap
        }
    }

    pub fn read_bit( &self, index: u32 ) -> bool {
        // read the bit
        let v = &self.map[(index / 8) as usize];
        let mask = self.get_mask( (index % 8) as u8 );
        return ( v & mask ) > 0;
    }

    pub fn set_bit( &mut self, index: u32, value: bool ) -> () {
        // set bit
        let mask = self.get_mask( (index % 8) as u8);
        let v = &mut self.map[(index / 8) as usize];
        match value {
            true => *v = *v | mask,
            false => *v = *v & !mask
        }

    }

    fn get_mask( &self, remainder: u8 ) -> u8 {
        return 128 as u8 >> remainder%8;
    }

    pub fn next_free ( &self, size: u32 ) -> Option<u32> {
        let mut index:u32 = 0;
        while index < self.map.len() as u32 {
            let mut free = true;
            let mut growsize:u32 = 0;
            while free {
                if growsize == size {
                    return Some(index);
                }
                free = self.read_bit(index + growsize);
                growsize = growsize + 1;
            }
            index = index + growsize;
        }
        return None;
    }
}
