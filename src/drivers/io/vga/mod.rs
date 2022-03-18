use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts::without_interrupts;

/*
This is the VGA driver. Will print to the standard VGA buffer at 0xb8000.
The `WRITER` handle prints yellow text to a black background.
*/

/// Prints to vga using `WRITER`
#[macro_export]
macro_rules! vga_print {
	($($arg:tt)*) => ($crate::drivers::io::vga::_print(format_args!($($arg)*)));
}

/// Prints to VGA using `WRITER` with a newline at the end.
#[macro_export]
macro_rules! vga_println {
	() => ($crate::vga_print!("\n"));
	($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}

/// Clear row
#[macro_export]
macro_rules! clear_row {
    () => {
        $crate::drivers::io::vga::_clear_row()
    };
}

#[macro_export]
macro_rules! clear_screen {
    () => {
        $crate::drivers::io::vga::_clear_screen()
    };
}

/// The real function to print to the VGA buffer using WRITER.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    without_interrupts(|| {
        WRITER
            .lock()
            .write_fmt(args)
            .expect("printing to vga failed")
    });
}

#[doc(hidden)]
pub fn _clear_row() {
    let mut wt = WRITER.lock();
    wt.clear_row(BUFFER_HEIGHT - 1);
    wt.column_position = 0;
}

#[doc(hidden)]
pub fn _clear_screen() {
    WRITER.lock().clear_screen()
}

/// VGA Color — C like Enum to define VGA compatible colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGrey = 0x07,
    DarkGrey = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0a,
    LightCyan = 0x0b,
    LightRed = 0x0c,
    Pink = 0x0d,
    Yellow = 0x0e,
    White = 0x0f,
}

/// Structure to hold a color code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// Creates a new instance of `ColorCode` from foreground and background `Color` Enums.
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// ScreenChar structure written to VGA buffer, contains character byte and `ColorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenChar {
    /// ascii character byte
    pub ascii_character: u8,
    /// color code for the color of this byte
    pub color_code: ColorCode,
}

/// BUFFER HEIGHT — The high of the VGA buffer.
pub const BUFFER_HEIGHT: usize = 25;
/// BUFFER_WIDTH — The width of the VGA buffer.
pub const BUFFER_WIDTH: usize = 80;

#[derive(Debug)]
#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug)]
pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                b'\t' => self.write_tab(),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_tab(&mut self) {
        self.write_byte(b' ');
        while !self.column_position % 4 == 0 {
            self.write_byte(b' ');
        }
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn clear_screen(&mut self) {
        for _ in 0..BUFFER_HEIGHT {
            self.new_line()
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[test_case]
fn vga_print() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts::without_interrupts;

    let s = "vga print test, single line";

    without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
