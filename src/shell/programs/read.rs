crate::include_lib!(std, io, fs);

pub fn main(args: Vec<String>) -> i32 {
    let mut fs = FILESYSTEM.lock();
    let mut code = 0;
    for arg in args {
        let (c, data) = fs.read_file(arg.clone());
        code = c as i32;
        match code {
            0 => {
                for b in data {
                    vga_print!("{}", b as char);
                }
            },
            1 => vga_println!("Error: file does not exist"),
            2 => vga_println!("Error: {} is not a file", arg),
            _ => vga_println!("Unknown error: {}", code),
        }
    }
    code
}
