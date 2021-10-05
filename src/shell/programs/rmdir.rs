crate::include_lib!(std, fs, io);

pub fn main(args: Vec<String>) -> Status {
    let mut fs = FILESYSTEM.lock();
    for d in args {
        let code = fs.remove_dir(d.clone());

        match code {
            Status::NotFound => vga_println!("Error: directory does not exist"),
            Status::WrongType => vga_println!("Error: '{}' is not a directory", d),
            Status::NotEmpty => vga_println!("Error: directory is not empty"),
            _ => {}
        }
    }
    Status::Success
}
