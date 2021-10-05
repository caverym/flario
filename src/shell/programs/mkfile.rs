crate::include_lib!(std, io, fs);

pub fn main(args: Vec<String>) -> Status {
    let mut fs = FILESYSTEM.lock();
    let mut code = Status::Success;
    for arg in args {
        code = fs.create_file(arg.clone());
        match code {
            Status::AlreadyExists => {
                vga_println!("Error: file {} already exists", arg);
            }
            _ => {
                vga_println!("Error: unknown error: {}", code);
            }
        }
    }
    code
}
