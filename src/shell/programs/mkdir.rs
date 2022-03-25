crate::include_lib!(std, io, fs);

pub fn main(args: Vec<String>) -> Status {
    let mut fs = FILESYSTEM.lock();

    for name in args {
        if fs.create_dir(&name).is_none() {
            return Status::FailedToWrite;
        }
    }

    Status::Success
}
