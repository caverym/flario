use super::vector::Vec;
use super::string::String;
use crate::shell::string::ToString;
use core::fmt::Formatter;

pub struct Command {
    pub arg_zero: CommandEN,
    pub args: Vec<String>,
}

pub enum CommandEN {
    Help,
    InitFs,
    About,
    Ls,
    Mkdir,
    Cat,
    Mkfile,
    Edit,
    NotFound(String),
}

impl core::fmt::Display for CommandEN {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}",
            match self {
                CommandEN::Help => "help",
                CommandEN::InitFs => "initfs",
                CommandEN::About => "about",
                CommandEN::Ls => "ls",
                CommandEN::Mkdir => "mkdir",
                CommandEN::Cat => "cat",
                CommandEN::Mkfile => "mkfile",
                CommandEN::Edit => "edit",
                CommandEN::NotFound(_) => "not found"
            }
        )
    }
}

impl From<String> for CommandEN {
    fn from(s: String) -> Self {
        let bytes: Vec<u8> = s.bytes();
        let s: &str = core::str::from_utf8(&bytes).unwrap_or("");
        match s {
            "help" => CommandEN::Help,
            "initfs" => CommandEN::InitFs,
            "about" => CommandEN::About,
            "ls" => CommandEN::Ls,
            "mkdir" => CommandEN::Mkdir,
            "cat" => CommandEN::Cat,
            "mkfile" => CommandEN::Mkfile,
            "edit" => CommandEN::Edit,
            _ => CommandEN::NotFound(s.to_string()),
        }
    }
}