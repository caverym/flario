use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => ($crate::drivers::io::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts::without_interrupts;

    without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("printing to serial failed")
    });
}

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}
