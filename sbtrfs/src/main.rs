extern crate fuse;
extern crate libc;
extern crate time;

mod maintenance;

use std::path::Path;
use libc::{ENOENT, ENOSYS};
use time::Timespec;
use std::env;
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};

struct SbFilesystem;

impl Filesystem for SbFilesystem {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        reply.error(ENOSYS);
    }
}

fn main2() {
    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };
    fuse::mount(SbFilesystem, &mountpoint, &[]);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // TODO: Something
        println!("Please use arguments");
        return;
    }

    match args[1].as_ref() {
        "mount"     => mount( Path::new( &args[2] ) ),
        "create"    => create( Path::new( &args[2] ) ),
        _           => panic!("only mount and create for now")
    }
}

fn mount( path:&Path ) {
    println!("disabled");
}

fn create( path:&Path) {
    let file = maintenance::InitialiseFS( &path );
}
