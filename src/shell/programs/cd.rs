use crate::kernel::fs::btfs::fs::NodeIdent;


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
    let mut fs = FILESYSTEM.lock();

    vga_println!("{}", path);

    if path == "/" {
        let mut env = ENVIRON.lock();
        env.update("cwd", path);
        return Some(());
    }

    let id = fs.flatten_ident(&NodeIdent::Name(path.to_string()))?;
    let fd = fs.open(NodeIdent::Id(id))?;

    if !fd.kind(&mut fs)? {
        return None;
    }

    let mut env = ENVIRON.lock();

    env.update("cwd", path);

    Some(())
}
