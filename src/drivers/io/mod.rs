pub mod keyboard;
pub mod serial;
pub mod vga;

#[macro_export]
macro_rules! vs_print {
	($($arg:tt)*) => ($crate::drivers::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vs_println {
	() => ($crate::vs_print!("\n"));
	($($arg:tt)*) => ($crate::vs_print!("{}\n", format_args!($($arg)*)));
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! dbg_vs_print {
	($($arg:tt)*) => ($crate::drivers::io::_print(format_args!($($arg)*)));
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! dbg_vs_println {
	() => ($crate::vs_print!("\n"));
	($($arg:tt)*) => ($crate::vs_print!("{}\n", format_args!($($arg)*)));
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! dbg_vs_print {
    ($($arg:tt)*) => {
        $crate::drivers::io::_nothing()
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! dbg_vs_println {
    () => {
        $crate::drivers::io::_nothing()
    };
    ($($arg:tt)*) => {
        $crate::drivers::io::_nothing()
    };
}

#[doc(hidden)]
pub fn _nothing() {}

#[doc(hidden)]
pub fn _print(fmt: core::fmt::Arguments) {
    vga::_print(fmt);
    serial::_print(fmt);
}
