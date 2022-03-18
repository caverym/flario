crate::include_lib!(std, io, time);

pub fn main(_: Vec<String>) -> Status {
    let now = Instant::now();
    let time = now.hours();

    vga_println!("{}:{}:{}\t{}", time.2, time.1, time.0, now);
    Status::Success
}
