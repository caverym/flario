use super::fs::NodeIdent;
use crate::kernel::{
    fs::{FileDescriptor, FileSystem, Inode},
    sc::Instant,
};
use alloc::{collections::BTreeMap, string::{String, ToString}, vec::Vec};

#[derive(Debug)]
pub enum Node {
    File(NodeContent, Vec<u8>),
    Directory(NodeContent, BTreeMap<u16, FileDescriptor<NodeIdent>>),
}

#[derive(Debug)]
pub struct NodeContent {
    name: String,
    ctime: Instant,
    mtime: Instant,
    dtime: Option<Instant>,
    id: u16,
}

impl Node {
    pub fn new(name: String, id: u16, kind: bool) -> Self {
        let now = Instant::now();

        if kind {
            Node::Directory(
                NodeContent {
                    name,
                    ctime: now,
                    mtime: now,
                    dtime: None,
                    id,
                },
                BTreeMap::new(),
            )
        } else {
            Node::File(
                NodeContent {
                    name,
                    ctime: now,
                    mtime: now,
                    dtime: None,
                    id,
                },
                Vec::new(),
            )
        }
    }

    pub fn content(&self) -> &NodeContent {
        match self {
            Node::File(c, _) => c,
            Node::Directory(c, _) => c,
        }
    }

    pub fn content_mut(&mut self) -> &mut NodeContent {
        match self {
            Node::File(c, _) => c,
            Node::Directory(c, _) => c,
        }
    }

    pub fn children(&mut self) -> Option<&mut BTreeMap<u16, FileDescriptor<NodeIdent>>> {
        match self {
            Node::File(_, _) => None,
            Node::Directory(_, c) => Some(c),
        }
    }

    pub fn data(&mut self) -> Option<&mut Vec<u8>> {
        match self {
            Node::File(_, d) => Some(d),
            Node::Directory(_, _) => None,
        }
    }

    pub fn mod_time(&self) -> Instant {
        self.content().mtime
    }

    pub fn create_time(&self) -> Instant {
        self.content().ctime
    }
}

impl NodeContent {
    pub fn new(name: String, id: u16) -> Self {
        let now = Instant::now();
        NodeContent {
            name,
            ctime: now,
            mtime: now,
            dtime: None,
            id,
        }
    }
}

impl Inode for Node {
    fn is_file(&self) -> bool {
        match self {
            Node::File(_, _) => true,
            _ => false,
        }
    }

    fn name(&self) -> String {
        let content = self.content();
        content.name.split("/").last().expect("file has invalid name").to_string()
    }

    fn id(&self) -> u16 {
        self.content().id
    }

    fn size(&self, fs: &impl FileSystem) -> usize {
        match self {
            Node::File(_, content) => content.len(),
            Node::Directory(_, ..) => fs.size(self.id()).unwrap(),
        }
    }

    fn is_deleted(&self) -> bool {
        self.content().dtime.is_some()
    }

    fn is_dir(&self) -> bool {
        !self.is_file()
    }
}
