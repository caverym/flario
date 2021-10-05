use super::string::String;
use super::vector::Vec;
use crate::kernel::status::Status;
use crate::shell::program::Program;
use alloc::boxed::Box;
use core::fmt::Formatter;
use core::pin::Pin;

pub struct Command {
    pub arg_zero: CommandEN,
    pub args: Vec<String>,
    func: Pin<Box<dyn Program>>,
}

impl Command {
    pub fn execute(self) -> Status {
        self.func.run(self.args)
    }
}

impl From<Vec<String>> for Command {
    fn from(mut args: Vec<String>) -> Self {
        let arg_zero: CommandEN = args.remove(0).into();
        let func = Pin::new(Box::new(match arg_zero {
            CommandEN::Help => super::programs::help::main,
            CommandEN::About => super::programs::about::main,
            CommandEN::Ls => super::programs::ls::main,
            CommandEN::Tree => super::programs::tree::main,
            CommandEN::Mkdir => super::programs::mkdir::main,
            CommandEN::Rmdir => super::programs::rmdir::main,
            CommandEN::Debug => super::programs::debug::main,
            CommandEN::Read => super::programs::read::main,
            CommandEN::Clear => super::programs::clear::main,
            CommandEN::Mkfile => super::programs::mkfile::main,
            CommandEN::Env => super::programs::env::main,
            CommandEN::Edit => super::programs::edit::main,
            CommandEN::Logo => super::programs::logo::main,
            CommandEN::NotFound => super::programs::not_found::main,
        }));

        Self {
            arg_zero,
            args,
            func,
        }
    }
}

#[derive(Clone)]
pub enum CommandEN {
    Help,
    About,
    Ls,
    Tree,
    Mkdir,
    Rmdir,
    Debug,
    Read,
    Clear,
    Mkfile,
    Env,
    Edit,
    Logo,
    NotFound,
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
                CommandEN::Tree => "tree",
                CommandEN::Mkdir => "mkdir",
                CommandEN::Rmdir => "rmdir",
                CommandEN::Debug => "debug",
                CommandEN::Clear => "clear",
                CommandEN::Read => "read",
                CommandEN::Mkfile => "mkfile",
                CommandEN::Edit => "edit",
                CommandEN::Logo => "logo",
                CommandEN::Env => "env",
                CommandEN::NotFound => "not found",
            }
        )
    }
}

impl From<String> for CommandEN {
    fn from(s: String) -> Self {
        let bytes: Vec<u8> = s.as_bytes().to_vec();
        let s: &str = core::str::from_utf8(&bytes).unwrap_or("");
        match s {
            "help" => CommandEN::Help,
            "about" => CommandEN::About,
            "ls" => CommandEN::Ls,
            "tree" => CommandEN::Tree,
            "mkdir" => CommandEN::Mkdir,
            "rmdir" => CommandEN::Rmdir,
            "debug" => CommandEN::Debug,
            "clear" => CommandEN::Clear,
            "read" => CommandEN::Read,
            "mkfile" => CommandEN::Mkfile,
            "edit" => CommandEN::Edit,
            "logo" => CommandEN::Logo,
            "env" => CommandEN::Env,
            _ => CommandEN::NotFound,
        }
    }
}
