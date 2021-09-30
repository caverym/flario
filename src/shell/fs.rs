use super::vector::Vec;
use super::string::String;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use crate::shell::string::ToString;

pub struct Filesystem(BTreeMap<String, Box<dyn Node>>);

struct File(Vec<u8>);

struct Directory(BTreeMap<String, Box<dyn Node>>);

trait Node {
    fn is_file(&self) -> bool;
    fn is_dir(&self) -> bool;
}

impl Node for File {
    fn is_file(&self) -> bool {
        true
    }

    fn is_dir(&self) -> bool {
        !self.is_file()
    }
}

impl Node for Directory {
    fn is_file(&self) -> bool {
        !self.is_dir()
    }

    fn is_dir(&self) -> bool {
        true
    }
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem(BTreeMap::new())
    }

    pub fn create_file(&mut self, name: &'static str) {
        self.0.insert(name.to_string(), Box::new(File(Vec::new())));
    }

    pub fn create_dir(&mut self, name: &'static str) {
        self.0.insert(name.to_string(), Box::new(Directory(BTreeMap::new())));
    }
}
