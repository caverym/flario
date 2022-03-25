crate::include_lib!(std, io, fs);

pub fn main(_: Vec<String>) -> Status {
    if rname().is_none() {
        Status::FailedToRead
    } else {
        Status::Success
    }
}

pub fn rname() -> Option<()> {
    let mut fs = FileSyetemRef::new();

    let map = fs.map()?;
    for (_, fd) in map {
        vga_println!("{}:{}", fd.name()?, fd.size()?);
    }

    Some(())
}
