pub mod debug;
pub mod env;
pub mod help;
pub mod logo;
pub mod ls;
pub mod mkdir;
pub mod mkfile;
pub mod read;
pub mod rmdir;
pub mod tree;

#[macro_export]
macro_rules! include_lib {
    ($($lib:ident),*) => {
        $(
            use $crate::shell::programs::includes::$lib::*;
        )*
    };
}

mod includes {
    pub mod std {
        pub use crate::shell::{
            string::{String, ToString},
            vector::Vec,
        };
    }

    pub mod io {
        pub use crate::{vga_print, vga_println};
    }

    pub mod fs {
        pub use crate::kernel::fs::{Node, FILESYSTEM};
    }

    pub mod env {
        pub use crate::shell::environ::{Environment, Key, ENVIRON};
    }
}
