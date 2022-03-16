crate::include_lib!(std, io, fs);

pub fn main(_: Vec<String>) -> Status {
    let fs = FILESYSTEM.lock();

    for n in fs.map() {
        vga_println!("{:#?}", n);
    }

    Status::Success
}
