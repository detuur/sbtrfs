use std::path::Path;
use libc::{ENOENT, ENOSYS};
use time::Timespec;
use fuse::{self, FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
use std::fs::{File, OpenOptions};
use super::inode;
use super::bitmap;

struct FuseBindFS {
    medium: File
}

impl Filesystem for FuseBindFS {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        reply.error(ENOSYS);
    }
}

pub fn init_mount( volume: &Path, mountpoint: &Path ) {
    
    let mut file = match  OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(true)
                           .open( volume  ) {
        Ok(file) => file,
        Err(_) => panic!("File at path {:?} can't be accessed", volume )
                           };
    
    let fs = FuseBindFS{ medium:  file };

    fuse::mount(fs, &mountpoint, &[]);
}
