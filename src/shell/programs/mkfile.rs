crate::include_lib!(std, io, fs);

pub fn main(args: Vec<String>) -> Status {
    let mut fs = FILESYSTEM.lock();

    if let Some(file) = fs.create_file() {
        vga_println!("{:?}", file);
    } else {
        vga_println!("failed to create file");
    }

    Status::Success
}
