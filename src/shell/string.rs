use super::vector::Vec;
use core::fmt::Formatter;
use core::slice::Split;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct String(Vec<u8>);

impl String {
    pub fn new() -> String {
        String(Vec::new())
    }

    pub fn from_bytes(bytes: &[u8]) -> String {
        String(bytes.to_vec())
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl<T> ToString for T where T: core::fmt::Display {
    fn to_string(&self) -> String {
        let a = format_args!("{}", self).as_str().unwrap_or("");
        let bytes: Vec<u8> = a.bytes().collect();
        String(bytes)
    }
}

impl core::fmt::Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let str = core::str::from_utf8(&self.0).unwrap_or("");
        write!(f, "{}", str)
    }
}
