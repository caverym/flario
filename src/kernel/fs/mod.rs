// pub mod vsfs; I don't feel like updating the API on a FS that crashes constantly
pub mod btfs;

use alloc::string::String;
use lazy_static::lazy_static;
use spin::Mutex;

use self::btfs::fs::NodeIdent;

type CurrentFileSystem = btfs::fs::BTFS;

lazy_static! {
    pub static ref FILESYSTEM: Mutex<CurrentFileSystem> = Mutex::new(CurrentFileSystem::new());
}

#[derive(Debug)]
pub struct FileDescriptor<T: Clone + Ord + PartialOrd + Eq + PartialEq>(T, bool);

impl FileDescriptor<NodeIdent> {
    pub fn new(id: NodeIdent, kind: bool) -> Self {
        FileDescriptor(id, kind)
    }

    pub fn id(&self, fs: &mut CurrentFileSystem) -> Option<u16> {
        fs.flatten_ident(&self.0)
    }

    pub fn name(&self, fs: &mut CurrentFileSystem) -> Option<String> {
        let id = fs.flatten_ident(&self.0)?;
        let node = fs.imap.get(&id)?;
        Some(node.name())
    }

    pub fn kind(&self, fs: &mut CurrentFileSystem) -> Option<bool> {
        let id = fs.flatten_ident(&self.0)?;
        let node = fs.imap.get(&id)?;
        Some(node.is_dir())
    }

    pub fn size(&self, fs: &mut CurrentFileSystem) -> Option<usize> {
        let id = fs.flatten_ident(&self.0)?;
        fs.size(id)
    }
}

pub trait FileSystem {
    type Identifier: Clone + Ord + PartialOrd + Eq + PartialEq;
    type File: Inode;
    type Directory: Inode;
    type ImapRef;

    fn next_free(&mut self) -> Option<u16>;

    fn map(&self) -> Option<Self::ImapRef>;

    fn create_file(&mut self, name: &str) -> Option<FileDescriptor<Self::Identifier>>;

    fn create_dir(&mut self, name: &str) -> Option<FileDescriptor<Self::Identifier>>;

    fn open(&mut self, ident: Self::Identifier) -> Option<FileDescriptor<Self::Identifier>>;

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
