use std::path::Path;
use libc::{ENOENT, ENOSYS};
use time::Timespec;
use fuse::{self, FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};

struct FuseBindFS;

impl Filesystem for FuseBindFS {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        reply.error(ENOSYS);
    }
}

pub fn init_mount( volume: &Path, mountpoint: &Path ) {
    fuse::mount(FuseBindFS, &mountpoint, &[]);
}
