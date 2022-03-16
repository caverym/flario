use core::{fmt::Display, sync::atomic::AtomicU16};

use crate::kernel::{sc::Instant, status::Status};

use alloc::{string::ToString, vec::Vec};

use super::{Filesystem, Inode};

#[derive(Debug)]
pub struct VSFS {
    imap: Vec<VFInode>,
    blocks: Vec<Option<Block>>,
}

#[derive(Debug)]
pub struct Imap(pub [Option<VFInode>; 16]);

impl VSFS {
    pub const fn new() -> Self {
        Self {
            imap: Vec::new(),
            blocks: Vec::new(),
        }
    }

    fn first_deleted(&self) -> Option<usize> {
        for (index, node) in self.imap.iter().enumerate() {
            if node.is_deleted() {
                return Some(index);
            }
        }

        None
    }

    pub fn next_imap(&mut self) -> Option<&mut VFInode> {
        if let Some(index) = self.first_deleted() {
            return Some(&mut self.imap[index]);
        }

        let len = self.imap.len();
        let node = VFInode::new(Mode(0b00000011_u8), NodeKind::File, self).ok()?;
        self.imap.insert(len, node);

        Some(&mut self.imap[len])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Block([u8; 4096]);

#[derive(Debug, Clone, Copy)]
pub struct VFInode {
    mode: Mode,
    ctime: Instant,
    mtime: Instant,
    dtime: Option<Instant>,
    id: u16,
    block: u16,
    size: u16,
    kind: NodeKind,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeKind {
    File,
    Directory,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NodeKind::Directory => write!(f, "dir"),
            NodeKind::File => write!(f, "file"),
        }
    }
}

impl VFInode {
    pub fn new(mode: Mode, kind: NodeKind, fs: &mut impl Filesystem) -> Result<VFInode, Status> {
        static IDGEN: AtomicU16 = AtomicU16::new(0);
        let id = IDGEN.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

        if kind == NodeKind::Directory {
            return Self::new_directory(mode, id, fs);
        }

        let now = Instant::now();
        let block = match fs.next_free() {
            Some(b) => b,
            None => return Err(Status::FailedToWrite),
        };

        Ok(VFInode {
            mode,
            ctime: now,
            mtime: now,
            dtime: None,
            id,
            block,
            size: 0,
            kind,
        })
    }

    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    fn new_directory(mode: Mode, id: u16, fs: &mut impl Filesystem) -> Result<VFInode, Status> {
        let now = Instant::now();

        Ok(VFInode {
            mode,
            ctime: now,
            mtime: now,
            dtime: None,
            id,
            block: id,
            size: 0,
            kind: NodeKind::Directory,
        })
    }
}

impl super::Inode for VFInode {
    fn is_file(&self) -> bool {
        self.kind == NodeKind::File
    }

    fn is_deleted(&self) -> bool {
        self.dtime.is_some()
    }

    fn name(&self) -> alloc::string::String {
        self.id().to_string()
    }

    fn id(&self) -> u16 {
        self.id
    }

    fn size(&self) -> usize {
        self.size as usize
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModeTypes {
    Read = 0b001,
    Write = 0b010,
    Execute = 0b100,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mode(u8);

impl Filesystem for VSFS {
    type File = VFInode;
    type Directory = VFInode;
    type ImapRef = Vec<VFInode>;

    fn next_free(&mut self) -> Option<u16> {
        for (index, block) in self.blocks.iter().enumerate() {
            if block.is_none() {
                return Some(index as u16);
            }
        }

        let len = self.blocks.len();
        self.blocks.insert(len, Some(Block([0u8; 4096])));

        Some(len as u16)
    }

    fn map(&self) -> Self::ImapRef {
        self.imap
            .clone()
            .iter()
            .filter_map(|n| if !n.is_deleted() { Some(*n) } else { None })
            .collect()
    }

    fn create_file(&mut self) -> Option<&mut Self::File> {
        if let Ok(inode) = VFInode::new(Mode(0b00000011_u8), NodeKind::File, self) {
            if let Some(node) = self.next_imap() {
                *node = inode;
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn create_dir(&mut self) -> Option<&mut Self::Directory> {
        if let Ok(inode) = VFInode::new(Mode(0b00000011_u8), NodeKind::Directory, self) {
            if let Some(node) = self.next_imap() {
                *node = inode;
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn open_file(&mut self) -> Self::File {
        todo!()
    }

    fn open_dir(&mut self) -> Self::Directory {
        todo!()
    }
}
