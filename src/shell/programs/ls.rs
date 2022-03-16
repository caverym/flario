use core::fmt::Display;
crate::include_lib!(std, io, fs);

pub fn main(_: Vec<String>) -> Status {
    let fs = FILESYSTEM.lock();

    vga_println!("Name\tType\tSize");
    for n in fs.map() {
        vga_println!(
            "{}\t{}\t{}",
            align("name", n.name()),
            align("type", n.kind()),
            align("size", n.size())
        );
    }

    Status::Success
}

fn align(row: &str, title: impl Display) -> String {
    let length = row.len();
    let mut disp = title.to_string();

    while length != disp.len() {
        disp.insert(0, ' ');
    }

    disp
}
