const MEMORY_SIZE: usize = 4096;
const FONTSET_START_ADDRESS: usize = 0x50;
const FONTSET_SIZE: usize = 80;
const START_ADDRESS: usize = 0x200;
const FONTSET_CHAR_SIZE: usize = 5;

pub struct Memory {
    buffer: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Self {
        Memory{
            buffer: [0; MEMORY_SIZE],
        }
    }

    pub fn load_fontset(&mut self, fontset: &[u8]) {
        let fontset_size = fontset.len();
        if fontset_size == FONTSET_SIZE {
            self.buffer[FONTSET_START_ADDRESS..FONTSET_START_ADDRESS + fontset_size]
                .copy_from_slice(&fontset);
        } else {
            panic!("Bad fontset");
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        if rom.len() <= MEMORY_SIZE - START_ADDRESS {
            self.buffer[START_ADDRESS..START_ADDRESS+ rom.len()].copy_from_slice(rom);
        } else {
            panic!("Program too big to fit in memory");
        }
    }

    pub fn get_opcode(&self, address: u16) -> u16 {
        let hi = self.buffer[address as usize] as u16;
        let lo = self.buffer[(address+1) as usize] as u16;
        hi << 8 | lo
    }

    pub fn get_char_addr(&self, digit: usize) -> u16 {
        (FONTSET_START_ADDRESS + (digit * FONTSET_CHAR_SIZE)) as u16
    }

    pub fn poke(&mut self, address: usize, value: u8) {
        self.buffer[address] = value;
    }

    pub fn peek(&self, address: usize) -> u8 {
        self.buffer[address]
    }
}