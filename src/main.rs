#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fario::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use fario::kernel::task::executor::Executor;
use fario::kernel::task::Task;
use fario::*;
entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    init();
    let mem_items = mem_init(&boot_info);
    let mut exe = Executor::new();

    // exe.spawn(Task::new(kernel::task::keyboard::print_keypresses()));
    exe.spawn(Task::new(welcome()));
    exe.spawn(Task::new(shell::main::shell()));

    exe.run();

    panic!("end of main!");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vs_println!("{}", info);
    halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    fario::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    use alloc::boxed::Box;

    let heapv1 = Box::new(41);
    let heapv2 = Box::new(13);
    assert_eq!(*heapv1, 41);
    assert_eq!(*heapv2, 13);
}

#[test_case]
fn large_vec() {
    use alloc::vec::Vec;

    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    use alloc::boxed::Box;
    use kernel::mem::globalloc::heap::HEAP_SIZE;

    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn test_println() {
    vs_print!("basic boot print test");
}
