use spin::MutexGuard;

use crate::kernel::fs::vsfs::VSFS;

crate::include_lib!(std, io, fs, env);

pub fn main(args: Vec<String>) -> Status {
    let fs = FILESYSTEM.lock();
    if args.len() == 0 {
        vga_println!("Usage: cd <path>");
        return Status::Success;
    }

    let path = args[0].clone();

    if path == "0" || path == "/" {
        let mut env = ENVIRON.lock();
        env.update("cwd", "/");
        return Status::Success;
    }

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

    find_from_root(&fs, &path)?;

    Status::Success
}

fn find_in_node(fs: &MutexGuard<VSFS>, _node: &impl Inode, path: &str) -> Status {
    if let Some(root) = fs.get_node(0) {
        let children = fs.node_children(root);

        for child in children {
            if child.is_file() {
                continue;
            };

            if child.name() == path {
                let mut env = ENVIRON.lock();
                env.update("cwd", &path);
                return Status::Success;
            }

            find_in_node(fs, child, path)?;
        }
    }

    Status::NotFound
}

fn find_from_root(fs: &MutexGuard<VSFS>, path: &str) -> Status {
    if let Some(root) = fs.get_node(0) {
        find_in_node(fs, root, path);
    }

    Status::NotFound
}
