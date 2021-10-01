crate::include_lib!(std, io);

pub(crate) async fn main(args: Vec<String>) -> i32 {
    if args.len() > 0 {
        for arg in args {
            vga_println!("{}", arg);
        }
    } else {
        vga_println!("HELP NOT IMPLEMENTED");
        return 1;
    }

    0
}
