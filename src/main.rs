#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flario::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use flario::kernel::mem::MemoryItems;
use flario::kernel::task::executor::Executor;
use flario::kernel::task::Task;
use flario::*;
// Defines entry point for the bootloader, bootloader defines_start function.
entry_point!(main);

/// The main entry point of the Flario kernel.
fn main(boot_info: &'static BootInfo) -> ! {
    init();
    let mem_items = mem_init(boot_info);

    let mut exe = Executor::new();
    exe.spawn(Task::new(welcome()));
    exe.spawn(Task::new(shell::main::shell()));
    exe.spawn(Task::new(print_memory(mem_items)));
    exe.run();
}

async fn print_memory(mem: MemoryItems) {
    serial_println!("{:#?}", mem);
}

/// Panic handler. Panics then halts the CPU indefinitely.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vs_println!("{}", info);
    halt();
}

/// Test Panic handler. runs `test_panic_handler`
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    flario::test_panic_handler(info)
}

/// Test allocation of memory
#[test_case]
fn simple_allocation() {
    use alloc::boxed::Box;

    let heapv1 = Box::new(41);
    let heapv2 = Box::new(13);
    assert_eq!(*heapv1, 41);
    assert_eq!(*heapv2, 13);
}

/// Test allocation of large vector
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

/// Test allocation of multiple boxes
#[test_case]
fn many_boxes() {
    use alloc::boxed::Box;
    use kernel::mem::globalloc::heap::HEAP_SIZE;

    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

/// Test VGA and Serial printing
#[test_case]
fn test_println() {
    vs_print!("basic boot print test");
}
