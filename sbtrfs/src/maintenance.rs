use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use byteorder::{BigEndian, WriteBytesExt};

pub fn initialise_fs( file_name: &Path ) -> File {
    let mut file = match  OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(true)
                           .open( file_name ) {
        Ok(file) => file,
        Err(_) => panic!("File at path '{:?}' can't be accesed", file_name)
    };

    write_super_block( &mut file );
    return file;

}

fn write_super_block ( file: &mut File ) {
    // Write our unique magic: 5(S)ecure Btrfs File 5(S)ystem
    let magic_bytes:u16 = 0x5BF5;
    file.write_u16::<BigEndian>( magic_bytes ).unwrap();
    file.set_len( 40*1024*1024 );   // 40MB
    println!("Just created a SuperBlock in the loaded file");
}
