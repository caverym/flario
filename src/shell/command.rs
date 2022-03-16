use super::string::String;
use super::vector::Vec;
use crate::kernel::status::Status;
use crate::shell::program::Program;
use alloc::boxed::Box;
use core::fmt::Formatter;
use core::pin::Pin;

pub struct Command {
    pub arg_zero: ArgZero,
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
        let arg_zero: ArgZero = args.remove(0).into();
        let func = Pin::new(Box::new(match arg_zero {
            ArgZero::Help => super::programs::help::main,
            ArgZero::About => super::programs::about::main,
            ArgZero::Ls => super::programs::ls::main,
            // ArgZero::Tree => super::programs::tree::main,
            // ArgZero::Mkdir => super::programs::mkdir::main,
            // ArgZero::Rmdir => super::programs::rmdir::main,
            ArgZero::Debug => super::programs::debug::main,
            // ArgZero::Read => super::programs::read::main,
            ArgZero::Clear => super::programs::clear::main,
            ArgZero::Mkfile => super::programs::mkfile::main,
            ArgZero::Env => super::programs::env::main,
            ArgZero::Logo => super::programs::logo::main,
            ArgZero::NotFound => super::programs::not_found::main,
            ArgZero::Time => super::programs::time::main,
        }));

        Self {
            arg_zero,
            args,
            func,
        }
    }
}

#[derive(Clone)]
pub enum ArgZero {
    Help,
    About,
    Ls,
    // Tree,
    //Mkdir,
    //Rmdir,
    Debug,
    //Read,
    Clear,
    Mkfile,
    Env,
    Logo,
    NotFound,
    Time,
}

impl core::fmt::Display for ArgZero {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArgZero::Help => "help",
                ArgZero::About => "about",
                ArgZero::Ls => "ls",
                // ArgZero::Tree => "tree",
                // ArgZero::Mkdir => "mkdir",
                // ArgZero::Rmdir => "rmdir",
                ArgZero::Debug => "debug",
                ArgZero::Clear => "clear",
                // ArgZero::Read => "read",
                ArgZero::Mkfile => "mkfile",
                ArgZero::Logo => "logo",
                ArgZero::Env => "env",
                ArgZero::NotFound => "not found",
                ArgZero::Time => "time",
            }
        )
    }
}

impl From<String> for ArgZero {
    fn from(s: String) -> Self {
        let bytes: Vec<u8> = s.as_bytes().to_vec();
        let s: &str = core::str::from_utf8(&bytes).unwrap_or("");
        match s {
            "help" => ArgZero::Help,
            "about" => ArgZero::About,
            "ls" => ArgZero::Ls,
            // "tree" => ArgZero::Tree,
            // "mkdir" => ArgZero::Mkdir,
            // "rmdir" => ArgZero::Rmdir,
            "debug" => ArgZero::Debug,
            "clear" => ArgZero::Clear,
            // "read" => ArgZero::Read,
            "mkfile" => ArgZero::Mkfile,
            "logo" => ArgZero::Logo,
            "env" => ArgZero::Env,
            "time" => ArgZero::Time,
            _ => ArgZero::NotFound,
        }
    }
}
