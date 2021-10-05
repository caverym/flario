crate::include_lib!(std, io);

pub fn main(_: Vec<String>) -> Status {
    clear_screen!();
    Status::Success
}