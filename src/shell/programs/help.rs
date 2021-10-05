crate::include_lib!(std, io);

pub(crate) async fn main(args: Vec<String>) -> Status {
    if !args.is_empty(){
        for arg in args {
            vga_println!("{}", arg);
        }
        Status::Success
    } else {
        vga_println!("HELP NOT IMPLEMENTED");
        Status::NotFound
    }
}
