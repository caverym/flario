crate::include_lib!(std, io, env);

pub fn main(_: Vec<String>) -> i32 {
    let env = ENVIRON.lock();
    let keys = env.keys();
    for key in keys {
        vga_println!("{}", key)
    }
    0
}
