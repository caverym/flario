crate::include_lib!(std, io, env);

pub fn main(_: Vec<String>) -> Status {
    let env = ENVIRON.lock();
    let keys = env.keys();
    for key in keys {
        vga_println!("{}", key)
    }
    Status::Success
}
