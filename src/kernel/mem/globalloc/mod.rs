use spin::{Mutex, MutexGuard};

pub mod fixed_size_list;
pub mod heap;
pub mod linked_list;

// #[global_allocator]
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

// #[global_allocator]
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

#[global_allocator]
static ALLOCATOR: Locked<fixed_size_list::FixedSizeAllocator> =
    Locked::new(fixed_size_list::FixedSizeAllocator::new());

pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    /*
    let remainder = addr % align;
    if remainder == 0 {
        addr
    } else {
        addr - remainder + align
    }
    */
    (addr + align - 1) & !(align - 1)
}
