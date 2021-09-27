use lazy_static::lazy_static;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

lazy_static! {
    pub static ref KEYBOARD_US104: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = {
        Mutex::new(Keyboard::new(
            layouts::Us104Key,
            ScancodeSet1,
            HandleControl::Ignore,
        ))
    };
}

lazy_static! {
    pub static ref KEYBOARD_UK105: Mutex<Keyboard<layouts::Uk105Key, ScancodeSet1>> = {
        Mutex::new(Keyboard::new(
            layouts::Uk105Key,
            ScancodeSet1,
            HandleControl::Ignore,
        ))
    };
}

lazy_static! {
    pub static ref KEYBOARD_AZERTY: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> = {
        Mutex::new(Keyboard::new(
            layouts::Azerty,
            ScancodeSet1,
            HandleControl::Ignore,
        ))
    };
}
