crate::include_lib!(std, io);

pub fn main(_: Vec<String>) -> i32 {
    vga_println!("{}", crate::FLARIO);
    0
}
