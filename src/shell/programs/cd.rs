crate::include_lib!(std, io, fs, env);

pub fn main(args: Vec<String>) -> Status {
    if args.len() == 0 {
        vga_println!("Usage: cd <path>");
        return Status::FailedToRead;
    }

    let path = args[0].to_string();

    if act_cd(&path).is_none() {
        Status::NotFound
    } else {
        Status::Success
    }
}

fn act_cd(path: &str) -> Option<()> {
    let mut fs = FileSyetemRef::new();
    let mut env = EnvironmentRef::new();

    vga_println!("{}", path);

    if path == "/" {
        env.update("cwd", path);
        return Some(());
    }

    let fd = fs.open(Identifier::Name(path.to_string()))?;

    if !fd.kind()? {
        return None;
    }

    env.update("cwd", path);

    Some(())
}
