use crate::{vga_println, vga_print};
use pc_keyboard::{Keyboard, ScancodeSet1, layouts, HandleControl, KeyCode, DecodedKey};
use pc_keyboard::layouts::Us104Key;
use crate::kernel::task::keyboard::ScancodeStream;
use super::fs::Filesystem;
use super::vector::Vec;
use super::string::String;
use crate::shell::command::{Command, CommandEN};
use futures_util::StreamExt;
use crate::shell::string::ToString;

struct Shell {
    keyboard: Keyboard<Us104Key, ScancodeSet1>,
    scancodes: ScancodeStream,
    code: i32,
    prompt: char,
    filesystem: Option<Filesystem>,
    last: Option<Command>,
}

impl Shell {
    pub fn new(
        keyboard: Keyboard<Us104Key, ScancodeSet1>,
        scancodes: ScancodeStream,
    ) -> Shell {
        Shell {
            keyboard,
            scancodes,
            code: 0,
            prompt: '>',
            filesystem: None,
            last: None
        }
    }

    pub async fn run(&mut self) {
        loop {
            let mut args = self.input().await;

            if args.is_empty() {
                continue;
            }

            let cmd = Command {
                arg_zero: args.remove(0).into(),
                args,
            };

           self.exe(cmd).await;
        }
    }

    pub async fn input(&mut self) -> Vec<String> {
        if self.code != 0 {
            vga_print!("[{}]", self.code);
        }

        vga_print!("{} ", self.prompt);

        let mut args: Vec<String> = Vec::new();
        let mut vc = Vec::new();

        loop {
            // get each key
            match self.key().await {
                // gathers each character, prints it to VGA buffer, inserts it at the end of `bytes`.
                Key::Char(c) => {
                    vga_print!("{}", c);
                    if c == ' ' {
                        args.insert(args.len(), String::from_bytes(&vc));
                        vc = Vec::new();
                    } else {
                        vc.insert(vc.len(), c as u8);
                    }
                }
                // enter, returns the function
                Key::Enter => {
                    vga_println!();
                    let s = String::from_bytes(&vc);
                    args.insert(args.len(), s);
                    break;
                },
                Key::Backspace => { vc.remove(vc.len() - 1); },
                // Don't do anything, unknown key
                Key::Other(_) => {}
                // Don't do anything, no key
                Key::None => {}
            }
        }

        args
    }

    pub async fn key(&mut self) -> Key {
        // Loop for every key press in scancode queue
        while let Some(scancode) = self.scancodes.next().await {
            // convert to key event
            if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
                // decode key event
                if let Some(key) = self.keyboard.process_keyevent(key_event) {
                    // matches which key was pressed and translates into `Key`
                    return match key {
                        DecodedKey::RawKey(key) => {
                            match key {
                                KeyCode::Backspace => Key::Backspace,
                                _ => Key::Other(key),
                            }
                        },
                        DecodedKey::Unicode(character) => match character {
                            '\n' => Key::Enter,
                            _ => Key::Char(character),
                        },
                    }
                }
            }
        }

        Key::None
    }

    pub async fn exe(&mut self, cmd: Command) {
        vga_println!("executing {}", cmd.arg_zero);
        self.code = match cmd.arg_zero {
            CommandEN::Help => help(cmd.args).await,
            CommandEN::InitFs => 1,
            CommandEN::About => 1,
            CommandEN::Ls => 1,
            CommandEN::Mkdir => 1,
            CommandEN::Cat => 1,
            CommandEN::Mkfile => 1,
            CommandEN::Edit => 1,
            CommandEN::NotFound(s) => not_found(s).await,
        }
    }
}

pub async fn shell() {
    // Initiate scancode stream
    let mut scancodes = ScancodeStream::new();
    // Create keyboard handle
    let mut keyboard = Keyboard::new(Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut shell = Shell::new(keyboard, scancodes);
    shell.run().await;

    vga_println!("Exiting Fario shell");
}

enum Key {
    Char(char),
    Enter,
    Backspace,
    Other(KeyCode),
    None,
}

async fn not_found(s: String) -> i32 {
    vga_println!("Command not found: {}", s);
    1
}

async fn help(args: Vec<String>) -> i32 {
    if args.len() > 0 {
        for arg in args {
            vga_println!("{}", arg);
        }
    } else {
        vga_println!("HELP NOT IMPLEMENTED");
        return 1;
    }

    0
}
