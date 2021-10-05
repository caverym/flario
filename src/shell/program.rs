use crate::kernel::status::Status;
use crate::shell::string::String;
use crate::shell::vector::Vec;

pub trait Program {
    fn run(&self, args: Vec<String>) -> Status;
}

impl<T> Program for T
where
    T: Fn(Vec<String>) -> Status,
{
    fn run(&self, args: Vec<String>) -> Status {
        self(args)
    }
}
