crate::include_lib!(std, io, fs);

pub fn main(args: Vec<String>) -> Status {
    let mut fs = FILESYSTEM.lock();
    for d in args {
        vga_println!("creating directory: '{}'", d);
        let code = fs.create_dir(&d);
        match code {
            Status::AlreadyExists => {
                vga_println!("Error: {} already exists", d);
                return code;
            }
            _ => vga_println!("Unknown error: {}", code),
        }
    }
    Status::Success
}
