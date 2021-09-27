#![no_std]
#![no_main]

use core::panic::PanicInfo;
use fario::{drivers::qemu, vs_println, Testable};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    vs_println!("[FAILED]");
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vs_println!("[OK]");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
    loop {}
}

fn should_panic() {
    assert!(false)
}
