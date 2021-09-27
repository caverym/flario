use crate::{vga_println, vga_print};
use pc_keyboard::KeyCode;
use crate::shell::vector::Vec;
use crate::shell::string::String;
use futures_util::StreamExt;
use crate::kernel::task::keyboard::ScancodeStream;
use pc_keyboard::{Keyboard, layouts, ScancodeSet1, HandleControl, DecodedKey};
use pc_keyboard::layouts::Us104Key;
use core::future::Future;

pub async fn shell() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    vga_println!("Fario shell");
    let mut code: i32 = 0;

    loop {
        if code != 0 {
            vga_print!("[{}]> ", code);
        } else {
            vga_print!("> ");
        }
        let bytes: Vec<u8> = input(&mut scancodes, &mut keyboard).await;
        let str = match core::str::from_utf8(&bytes) {
            Ok(s) => s,
            _ => { code = 2; break; },
        };
        vga_println!();


        if str == "maths" {
            code = maths(&mut scancodes, &mut keyboard).await;
        } else if !str.is_empty() {
            code = process(str).await;
        }

        if code == -1 {
            code = 0;
             break;
        }
    }

    vga_println!("exiting Fario shell... code: {}", code as u32);
}

async fn maths(scancodes: &mut ScancodeStream, keyboard: &mut Keyboard<Us104Key, ScancodeSet1>) -> i32 {
    enum Op {
        Add {
            lhs: i64,
            rhs: i64,
        },

        Sub {
            lhs: i64,
            rhs: i64,
        },

        Mul {
            lhs: i64,
            rhs: i64,
        },

        Div {
            lhs: i64,
            rhs: i64,
        },

        Noop,
    }

    loop {
        vga_print!("maths> ");

        let s = input(scancodes, keyboard).await;
        let s = match core::str::from_utf8(&s) {
            Ok(ss) => ss,
            _ => return 1,
        };

        if s == "exit" {
            break;
        }

        let svop: Vec<&str> = s.split(' ').collect();
        let mut op = Op::Noop;

        if svop.len() == 3 {
            op = match svop[1] {
                "+" => {
                    Op::Add {
                        lhs: match svop[0].parse() {
                            Ok(a) => a,
                            _ => { vga_println!("Unknown input: {}", svop[0]); continue; }
                        },
                        rhs: match svop[2].parse() {
                            Ok(a) => a,
                            _ => { vga_println!("Unknown input: {}", svop[0]); continue; }
                        },
                    }
                }
                "-" => {
                    Op::Sub {
                        lhs: match svop[0].parse() {
                            Ok(a) => a,
                            _ => { vga_println!("Unknown input: {}", svop[0]); continue; }
                        },
                        rhs: match svop[2].parse() {
                            Ok(a) => a,
                            _ => { vga_println!("Unknown input: {}", svop[0]); continue; }
                        },
                    }
                }
                "*" | "x" => {
                    Op::Mul {
                        lhs: match svop[0].parse() {
                            Ok(a) => a,
                            _ => { vga_println!("Unknown input: {}", svop[0]); continue; }
                        },
                        rhs: match svop[2].parse() {
                            Ok(a) => a,
                            _ => { vga_println!("Unknown input: {}", svop[0]); continue; }
                        },
                    }
                },
                "/" => {
                    Op::Div {
                        lhs: match svop[0].parse() {
                            Ok(a) => a,
                            _ => { vga_println!("Unknown input: {}", svop[0]); continue; }
                        },
                        rhs: match svop[2].parse() {
                            Ok(a) => a,
                            _ => { vga_println!("Unknown input: {}", svop[0]); continue; }
                        },
                    }
                }
                _ => Op::Noop,
            }
        }

        match op {
            Op::Add { lhs, rhs } => vga_println!("\n{}", lhs + rhs),
            Op::Sub { lhs, rhs } => vga_println!("\n{}", lhs + rhs),
            Op::Mul { lhs, rhs } => vga_println!("\n{}", lhs * rhs),
            Op::Div { lhs, rhs } => vga_println!("\n{}", lhs / rhs),
            Op::Noop => continue,
        }
    }

    vga_println!();

    0
}

async fn input(scancodes: &mut ScancodeStream, keyboard: &mut Keyboard<Us104Key, ScancodeSet1>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    loop {
        match get_key(scancodes, keyboard).await {
            Key::Enter => break,
            Key::Char(c) => {
                vga_print!("{}", c);
                bytes.insert(bytes.len(), c as u8);
            }
            Key::Other(_) => {}
            Key::None => {}
        }
    }

    bytes
}

enum Error {
    Parse,
}

async fn process(s: &str) -> i32 {
    match s {
        "help" => help(),
        "about" => about(),
        "exit" => exit(),
        "ls" => { vga_println!("Filesystems not implemented!"); 1 }
        _ => command_not_found(s),
    }
}

fn exit() -> i32 { -1 }

const ABOUT_HELP: &str =
"\
Fario About help.

Use `about` to view information about other commands or items.

Dictionary:
    Fario       View info about Fario kernel
    Shell       View info about Fario Shell
    Help        View help about Fario about
";

const ABOUT_SHELL: &str =
"\
Fario Shell.

Fario shell is a basic user shell implemented inside the Fario Kernel as a
task. The shell is in a Rust #![no_std] environment and is mostly a proof of
concept. I believe a working user interface, CLI, TUI, or GUI is required to
prove a working environment.

The shell implements all commands in itself as `Fn() -> i32`.
";

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

fn command_not_found(s: &str) -> i32 {
    vga_println!("Command `{}` not found!", s);
    1
}

const HELP_MSG: &str =
"\
Fario Shell Help.

Commands:
    help:        View help and commands,
    exit:        Exit Fario shell,
    about:       About the shell or other commands,
";

fn help() -> i32 {
    vga_println!("{}", HELP_MSG);
    0
}

enum Key {
    Char(char),
    Enter,
    Other(KeyCode),
    None,
}

async fn get_key(scancodes: &mut ScancodeStream, keyboard: &mut Keyboard<Us104Key, ScancodeSet1>) -> Key {
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::RawKey(key) => return Key::Other(key),
                    DecodedKey::Unicode(character) => {
                        match character {
                            '\n' => return Key::Enter,
                            _ => return Key::Char(character),
                        }
                    }
                }
            }
        }
    }

    Key::None
}