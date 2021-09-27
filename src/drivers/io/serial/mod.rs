use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

/// Serial print. Prints to serial port
#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => ($crate::drivers::io::serial::_print(format_args!($($arg)*)));
}

/// Serial print line. Prints to serial port with newline at end.
#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

/// Hidden function to print to Serial port.
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
    /// SERIAL1. A serial handle at port 0x3f8
    pub static ref SERIAL1: Mutex<SerialPort> = {
        // Create a mutable instance to the port.
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        // Initiates the port.
        serial_port.init();
        // Mutex box.
        Mutex::new(serial_port)
    };
}
