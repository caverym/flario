use super::string::String;
use super::vector::Vec;
use crate::shell::string::ToString;
use core::fmt::{Display, Formatter};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref ENVIRON: Mutex<Environment> = {
        let mut env = Environment::new();
        env.add("cwd".to_string(), "/".to_string())
            .expect("failed to initialize env");
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
        let mut str = "";
        let tab = " = ";

        str = format_args!("{}{}{}{}\n", str, self.name, tab, self.value)
            .as_str()
            .unwrap_or(str);

        write!(f, "{}", str)
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

    pub fn add(&mut self, name: String, value: String) -> Result<usize, u8> {
        let key: Key = Key::new(name, value);

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
