use core::fmt::{Display, Formatter};
use core::str::FromStr;
crate::include_lib!(std, io);

enum Item {
    FileSystem,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Item::FileSystem => "filesystem",
            }
        )
    }
}

impl Item {
    pub fn from_string(s: &String) -> Result<Self, ()> {
        if let Some(s) = core::str::from_utf8(&*s.bytes()).ok() {
            return match s {
                "filesystem" => Ok(Self::FileSystem),
                _ => Err(()),
            };
        }
        Err(())
    }
}

pub fn main(args: Vec<String>) -> i32 {
    for arg in args {
        if let Ok(i) = Item::from_string(&arg) {
            return match i {
                Item::FileSystem => debug_fs(),
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
