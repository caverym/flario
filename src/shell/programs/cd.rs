use spin::MutexGuard;
use crate::kernel::fs::Filesystem;
crate::include_lib!(std, io, fs, env);

struct CD<'a> {
    fs: MutexGuard<'a, Filesystem>,
    env: MutexGuard<'a, Environment>,
    dir: String,
}

impl CD<'_> {
    pub fn new() -> Self {
        CD {
            fs: FILESYSTEM.lock(),
            env: ENVIRON.lock(),
            dir: "/".to_string(),
        }
    }

    pub fn run(mut self, mut args: Vec<String>) -> Status {
        if !args.is_empty() {
            self.dir = args.remove(0);
        }
        let code = self.check_dir();

        match code {
            Status::Success => {
                self.set_env();
            },
            Status::NotFound => vga_println!("directory '{}' does not exist", self.dir),
            Status::WrongType => vga_println!("'{}' is not a directory", self.dir),
            _ => vga_println!("Unknown error: {}", code)
        }
        code
    }

    fn check_dir(&self) -> Status {
        let nodelist = self.fs.list_node(&self.env.value_of("cwd").unwrap_or_else(|| "/".to_string()));
        for (name, node) in nodelist.nodes {
            if *name == self.dir {
                if node.is_directory() {
                    return Status::Success;
                } else {
                    return  Status::WrongType;
                }
            }
        }
        Status::NotFound
    }

    fn set_env(&mut self) {
        self.env.update("cwd", &self.dir);
    }
}

pub fn main(args: Vec<String>) -> Status {
    CD::new().run(args)
}
