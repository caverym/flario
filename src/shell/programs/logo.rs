crate::include_lib!(std, io);

pub fn main(_: Vec<String>) -> Status {
    vga_println!("{}", crate::FLARIO);
    Status::Success
}
