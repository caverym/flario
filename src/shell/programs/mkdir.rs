crate::include_lib!(std, io, fs);

pub fn main(args: Vec<String>) -> i32 {
    let mut fs = FILESYSTEM.lock();
    for d in args {
        vga_println!("creating directory: '{}'", d);
        let code = fs.create_dir(d);
        return code as i32;
    }
    0
}
