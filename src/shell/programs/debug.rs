use core::fmt::{Display, Formatter};
crate::include_lib!(std, io);

enum Item {
    FileSystem,
    Environment,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Item::FileSystem => "filesystem",
                Item::Environment => "environment",
            }
        )
    }
}

impl Item {
    pub fn from_string(s: &String) -> Result<Self, ()> {
        return match s.as_str() {
            "filesystem" => Ok(Self::FileSystem),
            "environment" => Ok(Self::Environment),
            _ => Err(()),
        };
    }
}

pub fn main(args: Vec<String>) -> i32 {
    for arg in args {
        if let Ok(i) = Item::from_string(&arg) {
            return match i {
                Item::FileSystem => debug_fs(),
                Item::Environment => debug_env(),
            };
        } else {
            vga_println!("{} not an item or not yet implemented", arg);
            return 1;
        }
    }
    2
}

fn debug_fs() -> i32 {
    crate::include_lib!(fs);
    vga_println!("{:?}", FILESYSTEM.lock());
    0
}

fn debug_env() -> i32 {
    crate::include_lib!(env);
    vga_println!("{:?}", ENVIRON.lock());
    0
}