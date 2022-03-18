use spin::MutexGuard;

use crate::kernel::fs::vsfs::VSFS;

crate::include_lib!(std, io, fs, env);

pub fn main(_: Vec<String>) -> Status {
    let fs = FILESYSTEM.lock();
    let env = ENVIRON.lock();
    let _cwd = env.cwd();
    drop(env);
    vga_println!(".");
    let level = 1;
    if let Some(node) = fs.get_node(0) {
        let dirread = fs.node_children(node);
        for (name, node) in dirread.iter().enumerate() {
            tree_func(&name.to_string(), &fs, *node, level);
        }
    }
    Status::Success
}

fn tree_func(name: &str, fs: &MutexGuard<VSFS>, node: &impl Inode, mut level: i32) -> i32 {
    indentation_from_level(&level);
    if node.is_dir() {
        vga_println!("dir: {}", name);

        let dirread = fs.node_children(node);
        for (name, node) in dirread.iter().enumerate() {
            level += 1;
            tree_func(&name.to_string(), fs, *node, level);
            level -= 1;
        }
    } else if node.is_file() {
        vga_println!("file: {}", name);
    } else {
        vga_println!("unknown: {}", name);
    }

    level
}

fn indentation_from_level(level: &i32) {
    let mut i = 0;
    let mut line: String = String::new();

    while &i < level {
        i += 1;
        let mut bytes = line.as_bytes().to_vec();
        bytes.append(&mut [b' '; 4].to_vec());
        line = String::from_utf8(bytes).unwrap_or_default();
    }

    vga_print!("{}", line);
}
