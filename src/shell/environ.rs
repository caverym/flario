use super::string::String;
use super::vector::Vec;
use crate::shell::string::ToString;
use core::fmt::{Display, Formatter};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref ENVIRON: Mutex<Environment> = {
        let mut env = Environment::new();
        assert_eq!(env.add("cwd", "/")
            .expect("failed to initialize env"),
            0);
        Mutex::new(env)
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub name: String,
    pub value: String,
}

impl Key {
    pub fn new(name: String, value: String) -> Key {
        Key { name, value }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}  =  {}", self.name, self.value)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Environment(Vec<Key>);

impl Environment {
    pub fn new() -> Environment {
        Environment(Vec::new())
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.0.contains(key)
    }

    pub fn contains_entry(&self, key: &str) -> bool {
        for k in &self.0 {
            if k.name == *key {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, name: &str, value: &str) -> Result<usize, u8> {
        let key: Key = Key::new(name.to_string(), value.to_string());

        if self.contains_key(&key) || self.contains_entry(&key.name) {
            return Err(1);
        }

        let idx = self.0.len();

        self.0.insert(idx, key);

        Ok(idx)
    }

    pub fn keys(&self) -> &Vec<Key> {
        &self.0
    }
}
