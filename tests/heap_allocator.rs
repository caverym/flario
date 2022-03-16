#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flario::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use flario::kernel::mem::globalloc::heap::HEAP_SIZE;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use flario::*;
    init();
    let mem_items = mem_init(boot_info);

    test_main();
    halt();
}

#[test_case]
fn simple_allocation() {
    let heapv1 = Box::new(41);
    let heapv2 = Box::new(13);
    assert_eq!(*heapv1, 41);
    assert_eq!(*heapv2, 13);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let ll = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*ll, 1);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    flario::test_panic_handler(info)
}
