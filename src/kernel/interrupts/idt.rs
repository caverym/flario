use super::{pic, InterruptIndex};
use crate::{halt, kernel, vga_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt
            .set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_handler);
        idt.invalid_opcode.set_handler_fn(invalid_op_handler);
        idt.device_not_available
            .set_handler_fn(device_not_avail_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(kernel::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present
            .set_handler_fn(seg_not_present_handler);
        idt.stack_segment_fault
            .set_handler_fn(stack_seg_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(gen_protect_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floatp_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floatp_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception.set_handler_fn(security_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::Mouse.as_usize()].set_handler_fn(mouse_interrupt_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn mouse_interrupt_handler(_: InterruptStackFrame) {
    pic::end_of_interrupt(InterruptIndex::Mouse)
}

extern "x86-interrupt" fn security_handler(stack_frame: InterruptStackFrame, _: u64) {
    panic!("EXCEPTION: SECURITY:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: VIRTUALIZATION:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: MACHINE CHECK:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn simd_floatp_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: SIMD FLOAT POINT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn alignment_check_handler(stack_frame: InterruptStackFrame, _: u64) {
    panic!("EXCEPTION: ALIGNMENT CHECK:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn x87_floatp_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: X86 FLOATING POINT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn gen_protect_fault_handler(stack_frame: InterruptStackFrame, _: u64) {
    panic!("EXCEPTION: GENERAL PROTECTION:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn stack_seg_fault_handler(stack_frame: InterruptStackFrame, _: u64) {
    panic!("EXCEPTION: STACK SEGMENT FAULT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn seg_not_present_handler(_stack_frame: InterruptStackFrame, _: u64) {
    // vga_println!("EXCEPTION: SEGMENT NOT PRESENT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, _: u64) {
    panic!("EXCEPTION: INVALID TSS:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn device_not_avail_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DEVICE NOT AVAILABLE:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_op_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: INVALID OPERATION:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn bound_range_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BOUND RANGE EXCEEDED:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: OVERFLOW:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: NON-MASKABLE-INTERRUPT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    vga_println!("DEBUG INTERRUPT:\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    vga_println!("EXCEPTION: BREAKPOINT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE ERROR:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    pic::end_of_interrupt(InterruptIndex::Timer);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::kernel::task::keyboard::add_scancode(scancode);

    pic::end_of_interrupt(InterruptIndex::Keyboard)
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    vga_println!("EXCEPTION: PAGE FAULT");
    vga_println!("Accessed Address: {:?}", Cr2::read());
    vga_println!("Error Code: {:?}", error_code);
    vga_println!("{:#?}", stack_frame);
    halt();
}
