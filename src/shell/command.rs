use super::string::String;
use super::vector::Vec;
use crate::kernel::status::Status;
use core::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Command {
    pub arg_zero: ArgZero,
    pub args: Vec<String>,
}

impl Command {
    pub fn execute(self) -> Status {
        self.run()
    }

    pub fn run(self) -> Status {
        match self.arg_zero {
            ArgZero::Help => super::programs::help::main(self.args),
            ArgZero::About => super::programs::about::main(self.args),
            ArgZero::Ls => super::programs::ls::main(self.args),
            // ArgZero::Tree => super::programs::tree::main(self.args),
            ArgZero::Mkdir => super::programs::mkdir::main(self.args),
            // ArgZero::Rmdir => super::programs::rmdir::main(self.args),
            // ArgZero::Read => super::programs::read::main(self.args),
            ArgZero::Clear => super::programs::clear::main(self.args),
            ArgZero::Mkfile => super::programs::mkfile::main(self.args),
            ArgZero::Env => super::programs::env::main(self.args),
            ArgZero::Logo => super::programs::logo::main(self.args),
            ArgZero::NotFound => super::programs::not_found::main(self.args),
            ArgZero::Time => super::programs::time::main(self.args),
            ArgZero::Cd => super::programs::cd::main(self.args),
        }
    }
}

impl From<Vec<String>> for Command {
    fn from(mut args: Vec<String>) -> Self {
        let arg_zero: ArgZero = args.remove(0).into();

        Self { arg_zero, args }
    }
}

#[derive(Debug, Clone)]
pub enum ArgZero {
    Help,
    About,
    Ls,
    // Tree,
    Mkdir,
    //Rmdir,
    //Read,
    Clear,
    Mkfile,
    Env,
    Logo,
    NotFound,
    Time,
    Cd,
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
                ArgZero::Mkdir => "mkdir",
                // ArgZero::Rmdir => "rmdir",
                ArgZero::Clear => "clear",
                // ArgZero::Read => "read",
                ArgZero::Mkfile => "mkfile",
                ArgZero::Logo => "logo",
                ArgZero::Env => "env",
                ArgZero::NotFound => "not found",
                ArgZero::Time => "time",
                ArgZero::Cd => "cd",
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
            "mkdir" => ArgZero::Mkdir,
            // "rmdir" => ArgZero::Rmdir,
            "clear" => ArgZero::Clear,
            // "read" => ArgZero::Read,
            "mkfile" => ArgZero::Mkfile,
            "logo" => ArgZero::Logo,
            "env" => ArgZero::Env,
            "time" => ArgZero::Time,
            "cd" => ArgZero::Cd,
            _ => ArgZero::NotFound,
        }
    }
}
