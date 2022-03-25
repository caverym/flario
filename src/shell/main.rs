use super::string::String;
use super::vector::Vec;
use crate::kernel::status::Status;
use crate::kernel::task::keyboard::ScancodeStream;
use crate::shell::command::Command;
use crate::{vga_print, vga_println};
use futures_util::StreamExt;
use pc_keyboard::{layouts::Us104Key, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

struct Shell {
    keyboard: Keyboard<Us104Key, ScancodeSet1>,
    scancodes: ScancodeStream,
    code: Status,
    last: Option<Command>,
    prompt: char,
}

impl Shell {
    pub fn new(keyboard: Keyboard<Us104Key, ScancodeSet1>, scancodes: ScancodeStream) -> Shell {
        Shell {
            keyboard,
            scancodes,
            code: Status::Success,
            last: None,
            prompt: '>',
        }
    }

    pub async fn run(&mut self) {
        loop {
            let args = self.input().await;

            if args.is_empty() {
                continue;
            }

            if args[0] == "exit" {
                break;
            }

            let cmd: Command = args.into();
            self.last = Some(cmd.clone());

            self.exe(cmd).await;
        }
    }

    pub fn print_prompt(&self) {
        if self.code != Status::Success {
            vga_print!("[{}]", self.code);
        }

        vga_print!("{} ", self.prompt);
    }

    pub async fn input(&mut self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        let mut vc = Vec::new();

        self.print_prompt();

        loop {
            // get each key
            match self.key().await {
                Key::Backspace => {
                    if vc.is_empty() {
                    } else {
                        vc.remove(vc.len() - 1);
                        crate::clear_row!();
                        self.print_prompt();
                        vc.iter().for_each(|c| vga_print!("{}", *c as char));
                    }
                }
                // gathers each character, prints it to VGA buffer, inserts it at the end of `bytes`.
                Key::Char(c) => {
                    vga_print!("{}", c);
                    if c == ' ' {
                        args.insert(args.len(), String::from_utf8(vc).unwrap_or_default());
                        vc = Vec::new();
                    } else {
                        vc.insert(vc.len(), c as u8);
                    }
                }
                // enter, returns the function
                Key::Enter => {
                    vga_println!();
                    if vc.is_empty() {
                        self.print_prompt();
                        continue;
                    }
                    let s = String::from_utf8(vc).unwrap_or_default();
                    args.insert(args.len(), s);
                    break;
                }
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
                        DecodedKey::RawKey(key) => match key {
                            KeyCode::Backspace => Key::Backspace,
                            _ => Key::Other(key),
                        },
                        DecodedKey::Unicode(character) => match character {
                            '\n' => Key::Enter,
                            '\u{8}' => Key::Backspace,
                            _ => Key::Char(character),
                        },
                    };
                }
            }
        }
        Key::None
    }

    pub async fn exe(&mut self, cmd: Command) {
        self.code = cmd.execute();
    }
}

pub async fn shell() {
    vga_println!("Flario shell");
    // Initiate scancode stream
    let scancodes = ScancodeStream::new();
    // Create keyboard handle
    let keyboard = Keyboard::new(Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut shell = Shell::new(keyboard, scancodes);
    shell.run().await;

    vga_println!("Exiting flario shell");
}

enum Key {
    Char(char),
    Enter,
    Backspace,
    Other(KeyCode),
    None,
}
