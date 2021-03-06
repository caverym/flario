pub mod about;
pub mod clear;
pub mod env;
pub mod help;
pub mod logo;
pub mod ls;
pub mod mkdir;
pub mod mkfile;
pub mod not_found;
pub mod time;
//pub mod read;
//pub mod rmdir;
pub mod cd;
// pub mod tree;

#[macro_export]
macro_rules! include_lib {
    ($($lib:ident),*) => {
        $(
            #[allow(unused_imports)]
            use $crate::shell::programs::includes::$lib::*;
        )*
    };
}

mod includes {
    pub mod std {
        pub use crate::shell::programs::includes::stat::{Status, Termination};
        pub use crate::shell::programs::includes::str::{String, ToString};
        pub use crate::shell::programs::includes::vec::Vec;
    }

    pub mod stat {
        pub use crate::kernel::status::{Status, Termination};
    }

    pub mod fs {
        pub use crate::kernel::fs::{FileSystem, Inode, FileSyetemRef, filesystemref, Identifier};
    }

    pub mod vec {
        pub use crate::shell::vector::Vec;
    }

    pub mod str {
        pub use crate::shell::string::{String, ToString};
    }

    pub mod io {
        pub use crate::{clear_row, clear_screen, vga_print, vga_println};
    }

    pub mod time {
        pub use crate::kernel::sc::{Instant, SYSTEM_CLOCK};
    }

    pub mod env {
        pub use crate::kernel::environ::{Key, EnvironmentRef, environmentref};
    }
}
