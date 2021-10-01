crate::include_lib!(std, io, fs);

pub fn main(args: Vec<String>) -> i32 {
    let mut fs = FILESYSTEM.lock();
    let mut code = 0;
    for arg in args {
        code = fs.create_file(arg.clone()) as usize;
        match code {
            0 => {}
            1 => {
                vga_println!("Error: file {} already exists", arg);
                return code as i32;
            }
            _ => {
                vga_println!("Error: unknown error: {}", code);
                return code as i32;
            }
        }
    }
    code as i32
}
