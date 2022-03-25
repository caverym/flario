// pub mod vsfs; I don't feel like updating the API on a FS that crashes constantly
mod btfs;
pub mod public;
pub use public::*;

use alloc::{string::String, collections::BTreeMap};
use lazy_static::lazy_static;
use spin::Mutex;

use self::btfs::fs::NodeIdent;

pub type CurrentFileSystem = btfs::fs::BTFS;
pub type Identifier = NodeIdent;
pub type File = btfs::node::Node;
pub type Directory = btfs::node::Node;
pub type ImapRef = BTreeMap<u16, FileDescriptor<Identifier>>;

lazy_static! {
    static ref FILESYSTEM: Mutex<CurrentFileSystem> = Mutex::new(CurrentFileSystem::new());
}

#[derive(Debug)]
pub struct FileDescriptor<T: Clone + Ord + PartialOrd + Eq + PartialEq>(T, bool);

impl FileDescriptor<NodeIdent> {
    pub fn new(id: NodeIdent, kind: bool) -> Self {
        FileDescriptor(id, kind)
    }

    pub fn id(&self) -> Option<u16> {
        let mut fs = FILESYSTEM.lock();
        fs.flatten_ident(&self.0)
    }

    pub fn name(&self) -> Option<String> {
        let mut fs = FILESYSTEM.lock();
        let id = fs.flatten_ident(&self.0)?;
        let node = fs.imap.get(&id)?;
        Some(node.name())
    }

    pub fn kind(&self) -> Option<bool> {
        let mut fs = FILESYSTEM.lock();
        let id = fs.flatten_ident(&self.0)?;
        let node = fs.imap.get(&id)?;
        Some(node.is_dir())
    }

    pub fn size(&self) -> Option<usize> {
        let mut fs = FILESYSTEM.lock();
        let id = fs.flatten_ident(&self.0)?;
        fs.size(id)
    }
}

pub trait FileSystem {
    fn next_free(&mut self) -> Option<u16>;

    fn map(&self) -> Option<ImapRef>;

    fn create_file(&mut self, name: &str) -> Option<FileDescriptor<Identifier>>;

    fn create_dir(&mut self, name: &str) -> Option<FileDescriptor<Identifier>>;

    fn open(&mut self, ident: Identifier) -> Option<FileDescriptor<Identifier>>;

    fn size(&self, ident: u16) -> Option<usize>;
}

pub trait Inode {
    fn is_file(&self) -> bool;

    fn is_dir(&self) -> bool {
        !self.is_file()
    }

    fn name(&self) -> String;

    fn id(&self) -> u16;

    fn size(&self, fs: &impl FileSystem) -> usize;

    fn is_deleted(&self) -> bool;
}
