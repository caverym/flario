use crate::kernel::status::Status;
use crate::shell::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref ENVIRON: Mutex<Environment> = {
        let mut env = Environment::new();
        assert_eq!(env.add("cwd", "/").expect("failed to initialize env"), 0);
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

    pub fn cwd(&self) -> String {
        self.get("cwd").expect("environment does not contain cwd")
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

    pub fn add(&mut self, name: &str, value: &str) -> Result<usize, Status> {
        let key: Key = Key::new(name.to_string(), value.to_string());

        if self.contains_key(&key) || self.contains_entry(&key.name) {
            return Err(Status::AlreadyExists);
        } else {
            let idx = self.0.len();

            self.0.insert(idx, key);

            Ok(idx)
        }
    }

    pub fn update(&mut self, name: &str, value: &str) -> Status {
        let key: Key = Key::new(name.to_string(), value.to_string());

        if self.contains_entry(name) {
            for (idx, contkey) in self.0.iter().enumerate() {
                if key.name.clone() == contkey.name {
                    self.0[idx] = key;
                    break;
                }
            }
            Status::Success
        } else {
            Status::NotFound
        }
    }

    pub fn get(&self, name: &str) -> Option<String> {
        if self.contains_entry(name) {
            let keys = self.keys();
            for key in keys {
                if key.name == name {
                    return Some(key.value.clone());
                }
            }
        }
        None
    }

    pub fn keys(&self) -> &Vec<Key> {
        &self.0
    }
}
