crate::include_lib!(std, io);

pub fn main(args: Vec<String>) -> Status {
    vga_println!("ABOUT NOT IMPLEMENTED");
    Status::NotFound
}