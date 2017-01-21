

struct BitMap {
    map: Vec<u8>,
    offset: u32
}

impl BitMap {
    pub fn read_bit( &self, index: u32 ) -> bool {
        // read the bit
        let v = &map[index / 8];
        let mask = get_mask( index % 8 );
        return ( v & mask ) > 0;
    }

    pub fn set_bit( &self, index: u32, value: bool ) -> () {
        // set bit
        let v = &mut map[index / 8];
        let mask = get_mask( index % 8 );
        match value {
            true => *v = *v | mask;
            false => *v = *v & !mask;
        }

    }

    fn get_mask( &self, remainder: u8 ) -> u8 {
        return 128:u8 >> remainder%8;
    }

    pub next_free ( &self, size: u32 ) -> Option<u32> {
        let mut index = 0;
        while index < map.len() {
            let mut free = true;
            let mut growsize = 0;
            while free {
                if growsize == size
                    return Some(index);
                free = read_bit(index + growsize++);
            }
        }
        return None;
    }
}
