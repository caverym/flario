#![no_std]
#![no_main]

use core::panic::PanicInfo;
use flario::{drivers::qemu, vs_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    vs_println!("[FAILED]");
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vs_println!("[OK]");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

fn should_panic() {
    assert!(false)
}
