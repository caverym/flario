use super::node::{Node, NodeContent};
use crate::{kernel::{
    environ::ENVIRON,
    fs::{FileDescriptor, FileSystem, Inode},
}, vga_println};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::sync::atomic::AtomicU16;

#[derive(Debug)]
pub struct BTFS {
    pub imap: BTreeMap<u16, Node>,
}

impl BTFS {
    pub fn new() -> Self {
        let mut tree = BTreeMap::new();
        tree.insert(0, Node::new(String::from("/"), 0, true));
        Self { imap: tree }
    }

    pub fn flatten_ident(&self, id: &NodeIdent) -> Option<u16> {
        match id {
            NodeIdent::Root => Some(0),
            NodeIdent::Id(id) => Some(*id),
            NodeIdent::Name(id) => self.find_name(&id),
        }
    }

    fn find_name(&self, name: &str) -> Option<u16> {
        for (id, node) in &self.imap {
            if node.name() == name {
                return Some(*id);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeIdent {
    Root,
    Id(u16),
    Name(String),
}

impl FileSystem for BTFS {
    type Identifier = NodeIdent;
    type File = Node;
    type Directory = Node;
    type ImapRef = BTreeMap<u16, FileDescriptor<Self::Identifier>>;

    fn next_free(&mut self) -> Option<u16> {
        static NEXT_ID: AtomicU16 = AtomicU16::new(1);
        Some(NEXT_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }

    fn map(&self) -> Option<Self::ImapRef> {
        let cwd = ENVIRON.lock().cwd();
        let mut map: BTreeMap<u16, FileDescriptor<NodeIdent>> = BTreeMap::new();

        if cwd == "/" {
            let root_node = &self.imap[&0];
            if let Node::Directory(_, ref children) = root_node {
                for (child_id, child_fd) in children {
                    map.insert(*child_id, FileDescriptor(NodeIdent::Id(*child_id), child_fd.1));
                }
            } 
        } else {
            let path = if cwd.ends_with("/") {
                let mut tmp = cwd;
                tmp.remove(tmp.len()-1);
                tmp
            } else { cwd };

            let id = self.find_name(&path)?;
            let node = &self.imap[&id];
            if let Node::Directory(_, ref children) = node {
                for (child_id, child_fd) in children {
                    map.insert(*child_id, FileDescriptor(NodeIdent::Id(*child_id), child_fd.1));
                }
            } 
        }
        Some(map)
    }

    fn create_file(&mut self, name: &str) -> Option<FileDescriptor<Self::Identifier>> {
        let id = self.next_free()?;
        vga_println!("ID generated");
        let mut full = name.to_string();
        let file: &str = name.split("/").last()?;
        vga_println!("generated last: {}", file);
        full.remove_matches(file);
        vga_println!("removed matches: {}", full);

        let parent_id = self.find_name(&full)?;
        vga_println!("parent id: {}", parent_id);
        let parent = self.imap.get_mut(&parent_id)?;
        vga_println!("parent: {:?}", parent);
        let children = parent.children()?;
        vga_println!("parent's children: {:?}", children);
        children.insert(id, FileDescriptor(NodeIdent::Id(id), false));
        vga_println!("parent's children: {:?}", children);
        let node = Node::File(NodeContent::new(name.to_string(), id), Vec::new());
        self.imap.insert(id, node);
        Some(FileDescriptor(NodeIdent::Id(id), false))
    }

    fn create_dir(&mut self, name: &str) -> Option<FileDescriptor<Self::Identifier>> {
        let id = self.next_free()?;
        vga_println!("ID generated");
        let mut full = name.to_string();
        let file: &str = name.split("/").last()?;
        vga_println!("generated last: {}", file);
        full.remove_matches(file);
        vga_println!("removed matches: {}", full);

        let parent_id = self.find_name(&full)?;
        vga_println!("parent id: {}", parent_id);
        let parent = self.imap.get_mut(&parent_id)?;
        vga_println!("parent: {:?}", parent);
        let children = parent.children()?;
        vga_println!("parent's children: {:?}", children);
        children.insert(id, FileDescriptor(NodeIdent::Id(id), false));
        vga_println!("parent's children: {:?}", children);
        let node = Node::Directory(NodeContent::new(name.to_string(), id), BTreeMap::new());
        self.imap.insert(id, node);
        Some(FileDescriptor(NodeIdent::Id(id), false))
    }

    fn open(&mut self, id: Self::Identifier) -> Option<FileDescriptor<Self::Identifier>> {
        match id {
            NodeIdent::Root => Some(FileDescriptor(NodeIdent::Root, true)),
            NodeIdent::Id(id) => Some(FileDescriptor(NodeIdent::Id(id), self.imap.get(&id)?.is_dir())),
            NodeIdent::Name(_) => {
                let id = self.flatten_ident(&id)?;
                Some(FileDescriptor(NodeIdent::Id(id), self.imap.get(&id)?.is_dir()))
            },
        }
    }

    fn size(&self, ident: u16) -> Option<usize> {
        let node = &self.imap.get(&ident)?;
        match node {
            Node::File(_, v) => Some(v.len()),
            Node::Directory(_, t) => {
                let mut size = 0;
                for (_, n) in t {
                    let id = self.flatten_ident(&n.0)?;
                    if let Some(nodno) = self.imap.get(&id) {
                        size += nodno.size(self);
                    }
                }
                Some(size)
            }
        }
    }
}
