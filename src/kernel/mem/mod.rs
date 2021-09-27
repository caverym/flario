pub mod frame;
pub mod globalloc;
pub mod page;

use crate::kernel::mem::frame::BootInfoFrameAllocator;
use bootloader::BootInfo;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::VirtAddr;

#[derive(Debug)]
pub struct MemoryItems {
    pub offset_page_table: OffsetPageTable<'static>,
    pub frame_allocator: BootInfoFrameAllocator,
}

pub fn mem_init(boot_info: &'static BootInfo) -> MemoryItems {
    let mut offset_page_table =
        unsafe { page::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    globalloc::heap::init_heap(&mut offset_page_table, &mut frame_allocator)
        .expect("heap initialization failed");

    MemoryItems {
        offset_page_table,
        frame_allocator,
    }
}
