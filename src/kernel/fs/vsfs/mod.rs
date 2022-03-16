use core::sync::atomic::AtomicU16;

use crate::kernel::{sc::Instant, status::Status};

use alloc::{vec::Vec, boxed::Box};

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
            blocks:Vec::new(),
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
        self.imap.insert(
            len,
            node,
        );

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
#[derive(Debug, Clone, Copy)]
pub enum NodeKind {
    File,
    Directory
}

impl VFInode {
    pub fn new(mode: Mode, kind: NodeKind, fs: &mut impl Filesystem) -> Result<VFInode, Status> {
        static IDGEN: AtomicU16 = AtomicU16::new(0);
        let now = Instant::now();
        let id = IDGEN.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
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
}

impl super::Inode for VFInode {
    fn is_file(&self) -> bool {
        true
    }

    fn is_deleted(&self) -> bool {
        self.dtime.is_some()
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
        self.blocks.insert(
            len,
            Some(Block([0u8; 4096])),
        );

        Some(len as u16)
    }

    fn map(&self) -> Self::ImapRef {
        self.imap.clone().iter().filter_map(|n| if !n.is_deleted() { Some(*n) } else { None }).collect()
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
