crate::include_lib!(std, io, fs, env);

pub fn main(_: Vec<String>) -> Status {
    let fs = FILESYSTEM.lock();
    let env = ENVIRON.lock();
    let cwd = env.value_of("cwd").unwrap_or_else(|| String::from("/"));
    let dirread = fs.list_node(&cwd);
    for (name, node) in dirread.nodes {
        if node.is_directory() {
            vga_println!("dir: {}", name)
        } else if node.is_file() {
            vga_println!("file: {}", name)
        } else {
            vga_println!("unknown: {}", name)
        }
    }
    Status::Success
}
