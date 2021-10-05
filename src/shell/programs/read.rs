crate::include_lib!(std, io, fs);

pub fn main(args: Vec<String>) -> Status {
    let fs = FILESYSTEM.lock();
    let mut code = Status::Success;
    for arg in args {
        let (c, data) = fs.read_file(arg.clone());
        code = c;
        match code {
            Status::Success => {
                for b in data {
                    vga_print!("{}", b as char);
                }
            }
            Status::NotFound => vga_println!("Error: file does not exist"),
            Status::WrongType => vga_println!("Error: {} is not a file", arg),
            _ => vga_println!("Unknown error: {}", code),
        }
    }
    code
}
