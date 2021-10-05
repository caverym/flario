use crate::kernel::status::Status;
use crate::shell::string::{String, ToString};
use crate::shell::vector::Vec;

pub trait Program {
    fn name(&self) -> String;
    fn run(&self, args: Vec<String>) -> Status;
}

impl<T> Program for T
where
    T: Fn(Vec<String>) -> Status,
{
    fn name(&self) -> String {
        core::any::type_name::<T>().to_string()
    }

    fn run(&self, args: Vec<String>) -> Status {
        self(args)
    }
}
