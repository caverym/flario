use crate::kernel::task::keyboard::ScancodeStream;
use crate::shell::vector::Vec;
use crate::{vga_print, vga_println};
use futures_util::StreamExt;
use pc_keyboard::layouts::Us104Key;
use pc_keyboard::KeyCode;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

/// Main function for flario shell.
pub async fn shell() {
    // Initiate scancode stream
    let mut scancodes = ScancodeStream::new();
    // Create keyboard handle
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    // Print title
    vga_println!("flario shell");
    // Program wide error code.
    let mut code: i32 = 0;

    loop {
        // Print prompt, add error code if not 0
        if code != 0 {
            vga_print!("[{}]> ", code);
        } else {
            vga_print!("> ");
        }
        // Vector of bytes from user input.
        let bytes: Vec<u8> = input(&mut scancodes, &mut keyboard).await;
        // convert input into literal string, set error code to 2 if it fails to convert.
        let str = match core::str::from_utf8(&bytes) {
            Ok(s) => s,
            _ => {
                code = 2;
                break;
            }
        };
        // print newline at the end of the input
        vga_println!();

        // if requesting maths, run maths
        if str == "maths" {
            code = maths(&mut scancodes, &mut keyboard).await;
        } else if !str.is_empty() { // else run process hanlder and wait. get the process exit code.
            code = process(str).await;
        }

        // if code is -1, signals Shell to exit
        if code == -1 {
            code = 0;
            break;
        }
    }

    // print Exit message and code.
    vga_println!("exiting flario shell... code: {}", code as u32);
}

/// Maths shell. A shell inside flario shell to do simple math operations.
async fn maths(
    scancodes: &mut ScancodeStream,
    keyboard: &mut Keyboard<Us104Key, ScancodeSet1>,
) -> i32 {
    /// Define Operation enum, containing lhs and rhs numbers as i64.
    enum Op {
        /// Addition operation
        Add { lhs: i64, rhs: i64 },

        /// Subtraction operation
        Sub { lhs: i64, rhs: i64 },

        /// Multiplication operation
        Mul { lhs: i64, rhs: i64 },

        /// Division operation
        Div { lhs: i64, rhs: i64 },

        /// No operation operation
        Noop,
    }

    loop {
        // Print prompt
        vga_print!("maths> ");

        // gather user input as `Vec<u8>`
        let s = input(scancodes, keyboard).await;
        // Convert user input into literal string. Return 1 on failure.
        let s = match core::str::from_utf8(&s) {
            Ok(ss) => ss,
            _ => return 1,
        };

        // if user request to exit, exit. Code = 0
        if s == "exit" {
            break;
        }

        // split user input by spaces
        let svop: Vec<&str> = s.split(' ').collect();
        // create operation instance, set default to `Noop`
        let mut op = Op::Noop;

        // if input contains all parts, match input
        if svop.len() == 3 {
            op = match svop[1] {
                // User wants to add
                "+" => Op::Add {
                    lhs: match svop[0].parse() {
                        Ok(a) => a,
                        _ => {
                            vga_println!("Unknown input: {}", svop[0]);
                            continue;
                        }
                    },
                    rhs: match svop[2].parse() {
                        Ok(a) => a,
                        _ => {
                            vga_println!("Unknown input: {}", svop[0]);
                            continue;
                        }
                    },
                },
                // User wants to subtract
                "-" => Op::Sub {
                    lhs: match svop[0].parse() {
                        Ok(a) => a,
                        _ => {
                            vga_println!("Unknown input: {}", svop[0]);
                            continue;
                        }
                    },
                    rhs: match svop[2].parse() {
                        Ok(a) => a,
                        _ => {
                            vga_println!("Unknown input: {}", svop[0]);
                            continue;
                        }
                    },
                },
                // User wants to multiply
                "*" | "x" => Op::Mul {
                    lhs: match svop[0].parse() {
                        Ok(a) => a,
                        _ => {
                            vga_println!("Unknown input: {}", svop[0]);
                            continue;
                        }
                    },
                    rhs: match svop[2].parse() {
                        Ok(a) => a,
                        _ => {
                            vga_println!("Unknown input: {}", svop[0]);
                            continue;
                        }
                    },
                },
                // User wants to divide
                "/" => Op::Div {
                    lhs: match svop[0].parse() {
                        Ok(a) => a,
                        _ => {
                            vga_println!("Unknown input: {}", svop[0]);
                            continue;
                        }
                    },
                    rhs: match svop[2].parse() {
                        Ok(a) => a,
                        _ => {
                            vga_println!("Unknown input: {}", svop[0]);
                            continue;
                        }
                    },
                },
                // Noop
                _ => Op::Noop,
            }
        }

        // Match operation
        match op {
            // Add
            Op::Add { lhs, rhs } => vga_println!("\n{}", lhs + rhs),
            // Subtract
            Op::Sub { lhs, rhs } => vga_println!("\n{}", lhs + rhs),
            // Multiply
            Op::Mul { lhs, rhs } => vga_println!("\n{}", lhs * rhs),
            // Divide
            Op::Div { lhs, rhs } => {
                // if dividing by 0, refuse (impossible and panics)
                if rhs != 0 {
                    vga_println!("\n{}", lhs / rhs);
                } else {
                    vga_println!("Will not divide by 0");
                }
            }
            // Noop
            Op::Noop => continue,
        }
    }

    // Print new line at the end of operation.
    vga_println!();

    // return 0;
    0
}

/// Function to handle user input, returns when user presses enter.
async fn input(
    scancodes: &mut ScancodeStream,
    keyboard: &mut Keyboard<Us104Key, ScancodeSet1>,
) -> Vec<u8> {
    // crate empty vector of bytes.
    let mut bytes: Vec<u8> = Vec::new();

    loop {
        // get each key
        match get_key(scancodes, keyboard).await {
            // enter, returns the function
            Key::Enter => break,
            // gathers each character, prints it to VGA buffer, inserts it at the end of `bytes`.
            Key::Char(c) => {
                vga_print!("{}", c);
                bytes.insert(bytes.len(), c as u8);
            }
            // Don't do anything, unknown key
            Key::Other(_) => {}
            // Don't do anything, no key
            Key::None => {}
        }
    }

    // return Bytes;
    bytes
}

/// Handles execution of commands.
async fn process(s: &str) -> i32 {
    match s {
        "help" => help(),
        "about" => about(),
        "exit" => exit(),
        "ls" => {
            vga_println!("Filesystems not implemented!");
            1
        }
        _ => command_not_found(s),
    }
}

/// Exit command, signals shell to exit by using `-1`
fn exit() -> i32 {
    -1
}

/// About shell text for about program
const ABOUT_SHELL: &str = "\
flario Shell.

flario shell is a basic user shell implemented inside the flario Kernel as a
task. The shell is in a Rust #![no_std] environment and is mostly a proof of
concept. I believe a working user interface, CLI, TUI, or GUI is required to
prove a working environment.

The shell implements all commands in itself as `Fn() -> i32`.
";

/// About program
fn about() -> i32 {
    vga_println!("{}", ABOUT_SHELL);
    /*
    if vs.is_empty() {
        vga_println!("{}", ABOUT_HELP);
    } else {
        match vs[0] {
            "help" => vga_println!("{}", ABOUT_HELP),
            "shell" => vga_println!("{}", ABOUT_SHELL),
            _ => { vga_println!("Unknown item `{}`", vs[0]); return 1; }
        }
    }
     */

    0
}

/// Command not found, always returns 1.
fn command_not_found(s: &str) -> i32 {
    vga_println!("Command `{}` not found!", s);
    1
}

/// Help message for help program
const HELP_MSG: &str = "\
flario Shell Help.

Commands:
    help:        View help and commands,
    exit:        Exit flario shell,
    about:       About the shell or other commands,
";

/// Help program, always returns 0;
fn help() -> i32 {
    vga_println!("{}", HELP_MSG);
    0
}

/// Key enum
enum Key {
    Char(char),
    Enter,
    Other(KeyCode),
    None,
}

/// Returns `Key` on ever key press using scancodes and keyboard instances
async fn get_key(
    scancodes: &mut ScancodeStream,
    keyboard: &mut Keyboard<Us104Key, ScancodeSet1>,
) -> Key {
    // Loop for every key press in scancode queue
    while let Some(scancode) = scancodes.next().await {
        // convert to key event
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            // decode key event
            if let Some(key) = keyboard.process_keyevent(key_event) {
                // matches which key was pressed and translates into `Key`
                return match key {
                    DecodedKey::RawKey(key) => Key::Other(key),
                    DecodedKey::Unicode(character) => match character {
                        '\n' => Key::Enter,
                        _ => Key::Char(character),
                    },
                }
            }
        }
    }

    // return Key::None in the case of any strange failure.
    Key::None
}
