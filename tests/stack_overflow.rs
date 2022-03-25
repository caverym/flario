#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use flario::{drivers::qemu, vs_print, vs_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vs_print!("stack_overflow...");
    flario::init();
    init_test_idt();
    stack_overflow();
    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    flario::test_panic_handler(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(flario::kernel::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    vs_println!("[OK]");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}
