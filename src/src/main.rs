extern crate fuse;
extern crate libc;
extern crate time;
extern crate byteorder;

use std::path::Path;
use std::env;

mod maintenance;
mod fusebindings;

mod inode;
mod bitmap;
mod dir;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please use arguments");
        return;
    }

    match args[1].as_ref() {
        "mount"     => mount( Path::new(&args[2]), Path::new(&args[3]) ),
        "create"    => create( Path::new(&args[2]) ),
        _           => panic!("only mount and create for now")
    }
}

fn mount( volume_path:&Path, mountpoint:&Path ) {
    fusebindings::init_mount( volume_path, mountpoint );
}

fn create( path:&Path) {
    let file = maintenance::initialise_fs( &path );
}
