pub mod vsfs;

use alloc::string::String;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref FILESYSTEM: Mutex<vsfs::VSFS> = Mutex::new(vsfs::VSFS::new());
}

pub trait Filesystem {
    type File: Inode;
    type Directory: Inode;
    type ImapRef;

    fn next_free(&mut self) -> Option<u16>;

    fn map(&self) -> Self::ImapRef;

    fn create_file(&mut self) -> Option<&mut Self::File>;

    fn create_dir(&mut self) -> Option<&mut Self::Directory>;

    fn open_file(&mut self) -> Self::File;

    fn open_dir(&mut self) -> Self::Directory;
}

pub trait Inode {
    fn is_file(&self) -> bool;

    fn is_dir(&self) -> bool {
        !self.is_file()
    }

    fn name(&self) -> String;

    fn id(&self) -> u16;

    fn size(&self) -> usize;

    fn is_deleted(&self) -> bool;
}
