crate::include_lib!(std, io);

pub fn main(args: Vec<String>) -> Status {
    args.iter().for_each(|arg| vga_println!("Not found: {}", arg));
    Status::NotFound
}