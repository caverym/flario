crate::include_lib!(std, io);

pub fn main(_: Vec<String>) -> Status {
    vga_println!("ABOUT NOT IMPLEMENTED");
    Status::NotFound
}
