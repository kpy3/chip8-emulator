use std::fmt::{Debug, Formatter, Result};

pub const KEYPAD_SIZE: usize = 16;

#[derive(Copy, Clone)]
pub struct Keypad {
    keys: [bool; KEYPAD_SIZE]
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; KEYPAD_SIZE],
        }
    }

    pub fn press(&mut self, key: usize) {
        self.keys[key] = true;
    }

    pub fn pressed(&self, key: usize) -> bool {
        self.keys[key]
    }

}

impl Debug for Keypad {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("")
            .field("keys", &self.keys)
            .finish()
    }
}