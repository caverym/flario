use core::{fmt::{Display, Formatter}, ops::{Try, FromResidual, ControlFlow}};

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

impl FromResidual for Status {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        todo!()
    }
}

impl Try for Status {
    type Output = Status;

    type Residual = Status;

    fn from_output(output: Self::Output) -> Self {
        output
    }

    fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Status::Success => ControlFlow::Continue(self),
            Status::NotFound => ControlFlow::Break(self),
            Status::WrongType => ControlFlow::Break(self),
            Status::FailedToWrite => ControlFlow::Break(self),
            Status::FailedToRead => ControlFlow::Break(self),
            Status::AlreadyExists => ControlFlow::Break(self),
            Status::NotEmpty => ControlFlow::Break(self),
        }
    }
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
