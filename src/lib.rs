#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_main_entry);

pub mod drivers;
pub mod kernel;
pub mod shell;

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn halt_wr() {
    x86_64::instructions::hlt();
}

pub fn init() {
    kernel::gdt::init();
    kernel::interrupts::idt::init();
    kernel::interrupts::pic::init();
}

pub fn mem_init(boot_info: &'static BootInfo) -> kernel::mem::MemoryItems {
    kernel::mem::mem_init(boot_info)
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        vs_print!("{}...\t", core::any::type_name::<T>());
        self();
        vs_println!("[OK]")
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    use drivers::qemu::*;
    vs_println!("Running {} test(s)", tests.len());

    for test in tests {
        test.run()
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    use drivers::qemu::*;
    vs_println!("[failed]\n");
    vs_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

#[cfg(test)]
fn test_main_entry(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    loop {}
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}

const FLARIO: &str = ",------.,--.               ,--.
|  .---'|  | ,--,--.,--.--.`--' ,---.
|  `--, |  |' ,-.  ||  .--',--.| .-. |
|  |`   |  |\\ '-'  ||  |   |  |' '-' '
`--'    `--' `--`--'`--'   `--' `---' ";

#[cfg(not(debug_assertions))]
pub async fn welcome() {
    vga_println!(
        "{}\n\nversion {}-release by:\n{}",
        FLARIO,
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
}

#[cfg(debug_assertions)]
pub async fn welcome() {
    vga_println!(
        "{}\n\nversion {}-debug by:\n{}",
        FLARIO,
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
}
