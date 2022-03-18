crate::include_lib!(std, io, fs, env);

pub fn main(args: Vec<String>) -> Status {
    let fs = FILESYSTEM.lock();
    if args.len() == 0 {
        vga_println!("Usage: cd <path>");
        return Status::Success;
    }

    let path = args[0].clone();

    for node in fs.map() {
        if node.name() == path {
            if node.is_file() {
                vga_println!("{} is not a directory", path);
                return Status::NotFound;
            }

            let mut env = ENVIRON.lock();
            env.update("cwd", &path);
            return Status::Success;
        }
    }

    Status::Success
}
