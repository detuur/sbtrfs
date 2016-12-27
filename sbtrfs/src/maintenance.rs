use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn InitialiseFS( fileName: &Path ) -> File {
    let mut file = match  OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(true)
                           .open( fileName ) {
        Ok(file) => file,
        Err(err) => panic!("File at path '{:?}' can't be accesed", fileName)
    };
    
    WriteSuperBlock( &mut file );
    return file;

}

fn WriteSuperBlock ( fileHandle: &mut File ) {
    println!("Just created a SuperBlock in the loaded file");
}
