use core::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Success = 0,
    NotFound = 1,
    WrongType = 2,
    FailedToWrite = 3,
    FailedToRead = 4,
    AlreadyExists = 5,
    NotEmpty = 6,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.report())
    }
}

pub trait Termination: Sized {
    fn sys_report(self) -> u8;

    fn report(self) -> i32;
}

impl Termination for Status {
    fn sys_report(self) -> u8 {
        self as u8
    }

    fn report(self) -> i32 {
        self as i32
    }
}
