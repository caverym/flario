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
    pub fn from_string(s: &str) -> Result<Self, ()> {
        match s {
            "filesystem" => Ok(Self::FileSystem),
            "environment" => Ok(Self::Environment),
            _ => Err(()),
        }
    }
}

pub fn main(args: Vec<String>) -> Status {
    for arg in args {
        if let Ok(i) = Item::from_string(&arg) {
            match i {
                Item::FileSystem => debug_fs(),
                Item::Environment => debug_env(),
            };
        } else {
            vga_println!("{} not an item or not yet implemented", arg);
            return Status::NotFound;
        }
    }
    Status::Success
}

fn debug_fs() {
    crate::include_lib!(fs);
    vga_println!("{:?}", FILESYSTEM.lock());
}

fn debug_env() {
    crate::include_lib!(env);
    vga_println!("{:?}", ENVIRON.lock());
}
