crate::include_lib!(std, io, fs, env);

pub fn main(_: Vec<String>) -> Status {
    let fs = FILESYSTEM.lock();
    let env = ENVIRON.lock();
    let cwd = env.value_of("cwd").unwrap_or_else(|| String::from("/"));
    vga_println!(".");
    let level = 1;
    let dirread = fs.list_node(&cwd);
    for (name, node) in dirread.nodes {
        tree_func(name, node, level);
    }
    Status::Success
}

fn tree_func(name: &str, node: &Node, mut level: i32) -> i32 {
    indentation_from_level(&level);
    if node.is_directory() {
        vga_println!("dir: {}", name);

        if let Some(directory) = node.as_directory() {
            let nodes = directory.list_node();

            if !nodes.is_empty() {
                level += 1;

                for (name, node) in nodes {
                    level = tree_func(name, node, level);
                }

                level -= 1;
            }
        }
    } else if node.is_file() {
        vga_println!("file: {}", name);
    } else {
        vga_println!("unknown: {}", name);
    }

    level
}

fn indentation_from_level(level: &i32) {
    let mut i = 0;
    let mut line: String = String::new();

    while &i < level {
        i += 1;
        let mut bytes = line.as_bytes().to_vec();
        bytes.append(&mut [b' '; 4].to_vec());
        line = String::from_utf8(bytes).unwrap_or_default();
    }

    vga_print!("{}", line);
}
