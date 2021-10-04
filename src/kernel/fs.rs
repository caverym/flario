use crate::shell::string::String;
use alloc::collections::BTreeMap;
use alloc::prelude::v1::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref FILESYSTEM: Mutex<Filesystem> = {
        let mut fs: Filesystem = Filesystem::new();
        fs.write_readme();
        Mutex::new(fs)
    };
}

#[derive(Debug)]
pub struct Filesystem(BTreeMap<String, Node>);

#[derive(Debug)]
pub struct File {
    position: usize,
    data: Vec<u8>,
}

impl File {
    pub const fn new() -> File {
        File {
            position: 0,
            data: Vec::new(),
        }
    }

    pub fn write(&mut self, bytes: &[u8]) -> usize {
        let mut count: usize = 0;
        for byte in bytes {
            self.data.insert(self.position, *byte);
            self.position += 1;
            count += 1;
        }
        count
    }

    pub fn read(&self) -> (usize, Vec<u8>) {
        (self.data.len(), self.data.clone())
    }
}

#[derive(Debug)]
pub struct Directory(BTreeMap<String, Node>);

impl Directory {
    pub fn list_node(&self) -> Vec<(&String, &Node)> {
        let mut vd: Vec<(&String, &Node)> = Vec::new();
        for n in &self.0 {
            vd.insert(vd.len(), n)
        }
        vd
    }
}

#[derive(Debug)]
pub struct Node {
    directory: Option<Directory>,
    file: Option<File>,
}

impl Node {
    pub const fn new() -> Node {
        Node {
            directory: None,
            file: None,
        }
    }

    pub fn is_directory(&self) -> bool {
        self.directory.is_some()
    }

    pub fn is_file(&self) -> bool {
        self.file.is_some()
    }

    pub fn is_empty(&self) -> bool {
        !self.is_file() && !self.is_directory()
    }

    pub fn is_both(&self) -> bool {
        self.is_directory() && self.is_file()
    }

    pub fn as_directory(&self) -> &Option<Directory> {
        &self.directory
    }

    pub fn as_file(&self) -> &Option<File> {
        &self.file
    }
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem(BTreeMap::new())
    }

    fn write_readme(&mut self) {
        let readme: String = String::from_utf8(b"readme".to_vec()).unwrap_or(Default::default());
        self.create_file(readme.clone());
        self.write_to_file(
            readme,
            b"# flario shell

Welcome to flario shell. A shell integrated inside the flario kernel, written in
the Rust #![no_std] environment. The operating system also implements a
temporary filesystem using BTrees and nodes.

To view a list of commands, run `help`. To read some more about information, run `about`.

Thank you!
"
            .to_vec(),
        );
    }

    pub fn write_to_file(&mut self, name: String, data: Vec<u8>) -> u8 {
        if let Some(node) = self.0.get_mut(&name) {
            if let Some(ref mut f) = node.file {
                let code = f.write(&data);
                if code != data.len() {
                    // failed to write all requested bytes
                    3
                } else {
                    0
                }
            } else {
                // not a file
                2
            }
        } else {
            // File does not exist
            1
        }
    }

    pub fn create_file(&mut self, name: String) -> u8 {
        if self.0.contains_key(&name) {
            return 1;
        } else {
            let mut file = Node::new();
            file.file = Some(File::new());
            self.0.insert(name, file);
            0
        }
    }

    pub fn remove_file(&mut self, name: String) -> u8 {
        if let Some(node) = self.0.get(&name) {
            if let Some(f) = &node.file {
                if f.data.is_empty() {
                    self.0.remove_entry(&name);
                    0
                } else {
                    3
                }
            } else {
                2
            }
        } else {
            1
        }
    }

    pub fn list_node(&self) -> Vec<(&String, &Node)> {
        let mut vd: Vec<(&String, &Node)> = Vec::new();
        for n in &self.0 {
            vd.insert(vd.len(), n)
        }
        vd
    }

    pub fn create_dir(&mut self, name: String) -> u8 {
        if self.0.contains_key(&name) {
            1
        } else {
            let mut dir = Node::new();
            dir.directory = Some(Directory(BTreeMap::new()));
            self.0.insert(name, dir);
            0
        }
    }

    pub fn read_file(&self, name: String) -> (u8, Vec<u8>) {
        if let Some(node) = self.0.get(&name) {
            if let Some(ref f) = node.file {
                let (_, data) = f.read();
                (0, data)
            } else {
                // is not a file
                (2, Vec::new())
            }
        } else {
            // file does not exist
            (1, Vec::new())
        }
    }

    /// # Remove directory
    ///
    /// This function will delete an empty directory from the filesystem.
    ///
    /// ## Exit codes
    ///
    /// 0: Directory successfully deleted
    /// 1: The node requested does not exist
    /// 2: The node is not a directory
    /// 3: The directory is not empty
    pub fn remove_dir(&mut self, name: String) -> u8 {
        if let Some(node) = self.0.get(&name) {
            if let Some(d) = &node.directory {
                if d.0.is_empty() {
                    self.0.remove_entry(&name);
                } else {
                    // Directory not empty
                    return 3;
                }
            } else {
                // Node is not a directory
                return 2;
            }
        } else {
            // node does not exist
            return 1;
        }
        0
    }
}
