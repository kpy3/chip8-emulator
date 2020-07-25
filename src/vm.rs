use winit::event::{Event, VirtualKeyCode};
use crate::keypad;
use crate::keypad::Keypad;
use winit_input_helper::WinitInputHelper;

const KEYPAD_MAP: [(usize, VirtualKeyCode); keypad::KEYPAD_SIZE] = [
    (0, VirtualKeyCode::X),
    (1, VirtualKeyCode::Key1),
    (2, VirtualKeyCode::Key2),
    (3, VirtualKeyCode::Key3),
    (4, VirtualKeyCode::Q),
    (5, VirtualKeyCode::W),
    (6, VirtualKeyCode::E),
    (7, VirtualKeyCode::A),
    (8, VirtualKeyCode::S),
    (9, VirtualKeyCode::D),
    (10, VirtualKeyCode::Z),
    (11, VirtualKeyCode::C),
    (12, VirtualKeyCode::Key4),
    (13, VirtualKeyCode::R),
    (14, VirtualKeyCode::F),
    (15, VirtualKeyCode::V)
];

pub fn keypad(input: &WinitInputHelper) -> Keypad {
    let mut keypad = Keypad::new();
    for (key, key_code) in &KEYPAD_MAP {
        if input.key_pressed(*key_code) || input.key_held(*key_code) {
            keypad.press(*key);
        }
    }
    keypad
}