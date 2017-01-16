use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

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
    println!("Just created a SuperBlock in the loaded file");
}
