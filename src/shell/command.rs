use super::string::String;
use super::vector::Vec;
use crate::shell::string::ToString;
use core::fmt::Formatter;

#[derive(Clone)]
pub struct Command {
    pub arg_zero: CommandEN,
    pub args: Vec<String>,
}

#[derive(Clone)]
pub enum CommandEN {
    Help,
    About,
    Ls,
    Mkdir,
    Rmdir,
    Debug,
    Read,
    Clear,
    Mkfile,
    Edit,
    NotFound(String),
}

impl core::fmt::Display for CommandEN {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CommandEN::Help => "help",
                CommandEN::About => "about",
                CommandEN::Ls => "ls",
                CommandEN::Mkdir => "mkdir",
                CommandEN::Rmdir => "rmdir",
                CommandEN::Debug => "debug",
                CommandEN::Clear => "clear",
                CommandEN::Read => "read",
                CommandEN::Mkfile => "mkfile",
                CommandEN::Edit => "edit",
                CommandEN::NotFound(_) => "not found",
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
            "about" => CommandEN::About,
            "ls" => CommandEN::Ls,
            "mkdir" => CommandEN::Mkdir,
            "rmdir" => CommandEN::Rmdir,
            "debug" => CommandEN::Debug,
            "clear" => CommandEN::Clear,
            "read" => CommandEN::Read,
            "mkfile" => CommandEN::Mkfile,
            "edit" => CommandEN::Edit,
            _ => CommandEN::NotFound(s.to_string()),
        }
    }
}
