crate::include_lib!(std, fs, io);

pub fn main(args: Vec<String>) -> i32 {
    let mut fs = FILESYSTEM.lock();
    for d in args {
        let code = fs.remove_dir(d.clone());

        match code {
            1 => vga_println!("Error: directory does not exist"),
            2 => vga_println!("Error: '{}' is not a directory", d),
            3 => vga_println!("Error: directory is not empty"),
            _ => {}
        }

        return code as i32;
    }
    0
}
