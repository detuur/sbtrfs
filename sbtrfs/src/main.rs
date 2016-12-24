extern crate fuse;
extern crate libc;
extern crate time;

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

fn main() {
    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };
    fuse::mount(SbFilesystem, &mountpoint, &[]);
}
