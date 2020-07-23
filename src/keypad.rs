const KEYPAD_SIZE: usize = 16;

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

    pub fn release(&mut self, key: usize) {
        self.keys[key] = false;
    }

    pub fn pressed(&self, key: usize) -> bool {
        self.keys[key]
    }

}