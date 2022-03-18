#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(core_intrinsics)]
#![feature(try_trait_v2)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

pub use bootloader::{entry_point, BootInfo};

// Defines entry point for tests.
#[cfg(test)]
entry_point!(test_main_entry);

pub mod drivers;
pub mod kernel;
pub mod shell;
pub mod utils;

/// CPU Halt instruction. Halts CPU execution in an infinite loop.
pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// CPU Halt instruction. Halts CPU once.
pub fn halt_wr() {
    x86_64::instructions::hlt();
}

/// The `init()` function initiates the x86 CPU's GDT, IDT and PIC in order.
pub fn init() {
    kernel::gdt::init();
    kernel::interrupts::idt::init();
    kernel::interrupts::pic::init();
}

/// The `mem_init` function initiates memory, heap, and the global allocator. Returns a Memory item struct.
pub fn mem_init(boot_info: &'static BootInfo) -> kernel::mem::MemoryItems {
    kernel::mem::mem_init(boot_info)
}

/// Teastable trait, trait to run code tests
pub trait Testable {
    /// Run function for code tests.
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        vs_print!("{}...\t", core::any::type_name::<T>());
        self();
        vs_println!("[OK]")
    }
}

/// Test harness
pub fn test_runner(tests: &[&dyn Testable]) {
    use drivers::qemu::*;
    vs_println!("Running {} test(s)", tests.len());

    for test in tests {
        test.run()
    }

    exit_qemu(QemuExitCode::Success);
}

/// Panic handler when running tests, to exit QEMU using QEMU driver.
pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    use drivers::qemu::*;
    vs_println!("[failed]\n");
    vs_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

/// Entry point when testing.
#[cfg(test)]
fn test_main_entry(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    loop {}
}

/// Test breakpoint exception
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

/// Panic handler for Tests, runs `test_panic_handler`
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}

/// flario ascii logo
const FLARIO: &str = "\
,------.,--.               ,--.
|  .---'|  | ,--,--.,--.--.`--' ,---.
|  `--, |  |' ,-.  ||  .--',--.| .-. |
|  |`   |  |\\ '-'  ||  |   |  |' '-' '
`--'    `--' `--`--'`--'   `--' `---'\
";

/// Release welcome message
#[cfg(not(debug_assertions))]
pub async fn welcome() {
    vga_println!(
        "{}\n\nversion {}-release by:\n{}",
        FLARIO,
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
}

/// Debug welcome message
#[cfg(debug_assertions)]
pub async fn welcome() {
    vga_println!(
        "{}\n\nversion {}-debug by:\n{}",
        FLARIO,
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
}
