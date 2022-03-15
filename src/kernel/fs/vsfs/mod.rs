use std::time::Instant;

mod time {
    use core::time::Duration;

    pub struct Instant(Duration);

    
}

use alloc::vec::Vec;

use super::Filesystem;

pub struct VSFS {

}

pub enum Inode {
    FIle {
        mode: Mode,
        uid: u64,
        size: usize,
        data: Vec<u8>,
        ctime: Instant,
        mtime: Instant,
        dtime: Option<Instant>,

    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Read = 1,
    Write,
    Execute,
}

impl Filesystem for VSFS {
    type File;

    type OpenFile;

    type Directory;

    type OpenDirectory;

    fn create_file(&mut self) -> bool {
        todo!()
    }

    fn create_dir(&mut self) -> bool {
        todo!()
    }

    fn open_file(&mut self) -> Self::OpenFile {
        todo!()
    }

    fn open_dir(&mut self) -> Self::Directory {
        todo!()
    }
}