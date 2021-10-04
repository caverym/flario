pub mod debug;
pub mod help;
pub mod ls;
pub mod mkdir;
pub mod mkfile;
pub mod rmdir;
pub mod read;
pub mod logo;
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
        pub use crate::kernel::fs::FILESYSTEM;
    }
}
