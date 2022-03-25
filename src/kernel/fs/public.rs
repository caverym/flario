use super::{CurrentFileSystem, FileSystem, Identifier, ImapRef, FILESYSTEM, FileDescriptor};

pub struct FileSyetemRef;

impl FileSyetemRef {
    pub fn new() -> Self {
        Self
    }

    pub fn map(&self) -> Option<ImapRef> {
        super::FILESYSTEM.lock().map()
    }

    pub fn create_file(&self, name: &str) -> Option<FileDescriptor<Identifier>> {
        FILESYSTEM.lock().create_file(name)
    }

    pub fn create_dir(&self, name: &str) -> Option<FileDescriptor<Identifier>> {
        FILESYSTEM.lock().create_dir(name)
    }

    pub fn open(&self, ident: Identifier) -> Option<FileDescriptor<Identifier>> {
        FILESYSTEM.lock().open(ident)
    }

    pub fn size(&self, ident: u16) -> Option<usize> {
        FILESYSTEM.lock().size(ident)
    }
}

pub fn filesystemref() -> FileSyetemRef {
    FileSyetemRef::new()
}
