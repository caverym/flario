crate::include_lib!(std, io, fs);

pub fn main(_: Vec<String>) -> i32 {
    let fs = FILESYSTEM.lock();
    for (name, node) in fs.list_node() {
        if node.is_directory() {
            vga_println!("dir: {}", name)
        } else if node.is_file() {
            vga_println!("file: {}", name)
        } else {
            vga_println!("unknown: {}", name)
        }
    }
    0
}
