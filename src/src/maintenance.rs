use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use byteorder::{BigEndian, WriteBytesExt};
use super::dir::{Dir, DirEntry};
use std::io::prelude::*;
use super::fusebindings::FuseBindFS;

pub fn initialise_fs( file_name: &Path ) {
    let mut file = match  OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(true)
                           .open( file_name ) {
        Ok(file) => file,
        Err(_) => panic!("File at path {:?} can't be accessed", file_name)
    };

    write_super_block( &mut file );
    let mut fs = FuseBindFS::new( file );
    fs.imap.set_bit( 0, false );
    let first_ino = fs.make_dir( 0, 0, 0o777, 0 );
    println!("Created ino {}", first_ino);
}

fn write_super_block ( file: &mut File ) {
    // Write our unique magic: 5(S)ecure Btrfs File 5(S)ystem
    let magic_bytes:u16 = 0x5BF5;
    file.write_u16::<BigEndian>( magic_bytes ).unwrap();

    file.set_len( 40*1024*1024 );   // TODO: Not make this hardcoded to 40MB
    println!("Just created a SuperBlock in the loaded file");
}
