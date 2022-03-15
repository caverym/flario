mod vsfs;

pub trait Filesystem {
    type File: Inode;
    type OpenFile;
    type Directory: Inode;
    type OpenDirectory;

    fn create_file(&mut self) -> bool;

    fn create_dir(&mut self) -> bool;

    fn open_file(&mut self) -> Self::OpenFile;

    fn open_dir(&mut self) -> Self::Directory;
}

pub trait Inode {
    fn is_file(&self) -> bool;

    fn is_dir(&self) -> bool {
        !self.is_file()
    }

    fn read(&mut self, data: &mut &[u8]) -> usize;

    fn write(&mut self, data: &[u8]) -> usize;

    fn seek(&mut self, pos: usize) -> usize;

    fn position(&self) -> usize;
}
