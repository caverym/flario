use super::vector::Vec;
use core::fmt::Formatter;

pub struct String {
    inner: Vec<u8>,
}

impl String {
    pub fn new() -> String {
        String {
            inner: Vec::new(),
        }
    }

    pub fn from_bytes(b: &[u8]) -> String {
        String {
            inner: b.to_vec(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.inner.clone()
    }
}
