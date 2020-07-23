use rand::Rng;
use rand::rngs::ThreadRng;
use crate::memory::Memory;
use crate::display::Display;
use crate::stack::Stack;
use crate::keypad::Keypad;

const REGISTER_COUNT: usize = 16;
const START_ADDRESS: usize = 0x200;

pub struct Chip8 {
    v: [u8; REGISTER_COUNT],
    memory: Memory,
    stack: Stack,
    display: Display,
    rng: ThreadRng,

    index: u16,
    // program counter
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            v: [0; REGISTER_COUNT],
            memory: Memory::new(),
            index: 0,
            pc: START_ADDRESS as u16,
            stack: Stack::new(),
            delay_timer: 0,
            sound_timer: 0,
            keypad: Keypad::new(),
            display: Display::new(),
            rng: rand::thread_rng()
        }
    }

    pub fn load_fontset(&mut self, fontset: &Vec<u8>) {
        self.memory.load_fontset(fontset);
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.memory.load_rom(rom);
    }

    pub fn draw(&mut self, buf: &mut [u8]) {
        self.display.draw(buf);
    }

    pub fn tick(&mut self) {
        let opcode = self.memory.get_opcode(self.pc);
        self.pc += 2;
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as u16;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as u8;

        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8x06(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8x0e(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxkk(x, kk),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _ => panic!("Unknown opcode: {} at {}", opcode, self.pc)
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    // CLS
    // Clear screen
    fn op_00e0(&mut self) {
        self.display.clear();
    }

    // RET
    // Return from subroutine
    fn op_00ee(&mut self) {
        self.pc = self.stack.pop();
    }

    // JMP addr
    // Jump to location nnn
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // CALL addr
    // Call subroutine at nnn
    fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }

    // SE Vx, byte
    // Skip next instruction if Vx = kk
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 2;
        }
    }

    // SNE Vx, byte
    // Skip next instruction if Vx != kk
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 2;
        }
    }

    // SE Vx, Vy
    // Skip next instruction if Vx = Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    // LD Vx, byte
    // Set Vx = kk
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    // ADD Vx, byte
    // Set Vx = Vx + kk
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = self.v[x].wrapping_add(kk);
    }

    // LD Vx, Vy
    // Set Vx = Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    // OR Vx, Vy
    // Set Vx = Vx OR Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    // AND Vx, Vy
    // Set Vx = Vx AND Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    // XOR Vx, Vy
    // Set Vx = Vx XOR Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    // ADD Vx, Vy
    // Set Vx = Vx + Vy, set VF = carry.
    //
    // The values of Vx and Vy are added together. If the result is
    // greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    // Only the lowest 8 bits of the result are kept, and stored in Vx
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let sum: u16 = self.v[x] as u16 + self.v[y] as u16;
        if sum > 255 {
            self.v[0xf] = 1;
        } else {
            self.v[0xf] = 0;
        }
        self.v[x] = sum as u8;
    }

    // SUB Vx, Vy
    // Set Vx = Vx - Vy, set VF = NOT borrow.
    //
    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted
    // from Vx, and the results stored in Vx.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        if self.v[x] > self.v[y] {
            self.v[0xf] = 1;
        } else {
            self.v[0xf] = 0;
        }
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }

    // SHR Vx
    // Set Vx = Vx SHR 1.
    //
    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    // Then Vx is divided by 2.
    fn op_8x06(&mut self, x: usize) {
        self.v[0xf] = self.v[x] & 0x1;
        self.v[x] >>= 1;
    }

    // SUBN Vx, Vy
    // Set Vx = Vy - Vx, set VF = NOT borrow.
    //
    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy,
    // and the results stored in Vx.
    fn op_8xy7(&mut self, x: usize, y: usize) {
        if self.v[y] > self.v[x] {
            self.v[0xf] = 1;
        } else {
            self.v[0xf] = 0;
        }
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    }

    // SHL Vx {, Vy}
    // Set Vx = Vx SHL 1.
    //
    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    // Then Vx is multiplied by 2.
    fn op_8x0e(&mut self, x: usize) {
        // Save MSB in VF
        self.v[0x0f] = (self.v[x] & 0x80) >> 7;
        self.v[x] <<= 1;
    }

    // SNE Vx, Vy
    // Skip next instruction if Vx != Vy.
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    // LD I, addr
    // Set I = nnn.
    fn op_annn(&mut self, nnn: u16) {
        self.index = nnn;
    }

    // JP V0, addr
    // Jump to location nnn + V0.
    fn op_bnnn(&mut self, nnn: u16) {
        self.pc = self.v[0] as u16 + nnn;
    }

    // RND Vx, byte
    // Set Vx = random byte AND kk.
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        self.v[x] = self.rng.gen::<u8>() & kk;
    }

    // DRW Vx, Vy, n
    // The interpreter reads n bytes from memory, starting at the address
    // stored in I. These bytes are then displayed as sprites on screen at
    // coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
    // If this causes any pixels to be erased, VF is set to 1, otherwise
    // it is set to 0. If the sprite is positioned so part of it is outside
    // the coordinates of the display, it wraps around to the opposite side
    // of the screen.
    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) {
        self.v[0xf] = 0;

        for byte in 0..n as usize {
            let y_pos = (self.v[y] as usize + byte) % self.display.height();
            let sprite_byte = self.memory.peek(self.index as usize + byte);
            for bit in 0..=7 {
                let x_pos = (self.v[x] as usize + bit) % self.display.width();

                let sprite_pixel = (sprite_byte >> (7 - bit)) & 1;
                let screen_pixel = self.display.peek(x_pos, y_pos);
                self.v[0xf] |= sprite_pixel & screen_pixel;
                self.display.poke(x_pos, y_pos, screen_pixel ^ sprite_pixel);
            }
        }
    }

    // SKP Vx
    // Skip next instruction if key with the value of Vx is pressed.
    fn op_ex9e(&mut self, x: usize) {
        let key = self.v[x] as usize;
        if self.keypad.pressed(key) {
            self.pc += 2;
        }
    }

    // SKNP Vx
    // Skip next instruction if key with the value of Vx is not pressed.
    fn op_exa1(&mut self, x: usize) {
        let key = self.v[x] as usize;
        if !self.keypad.pressed(key) {
            self.pc += 2;
        }
    }

    // LD Vx, DT
    // Set Vx = delay timer value.
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    // LD Vx, K
    // Wait for a key press, store the value of the key in Vx.
    fn op_fx0a(&mut self, x: usize) {
        for k in 0..16 as u8 {
            if self.keypad.pressed(k as usize) {
                self.v[x] = k;
                return;
            }
        }
        self.pc -= 2;
    }

    // LD DT, Vx
    // Set delay timer = Vx.
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }

    // LD ST, Vx
    // Set sound timer = Vx.
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }

    // ADD I, Vx
    // Set I = I + Vx.
    fn op_fx1e(&mut self, x: usize) {
        self.index += self.v[x] as u16;
        if self.index > 255 {
            self.v[0x0f] = 1;
        } else {
            self.v[0x0f] = 0
        }
    }

    // LD F, Vx
    // Set I = location of sprite for digit Vx.
    fn op_fx29(&mut self, x: usize) {
        self.index = self.memory.get_char_addr(x);
    }

    // LD B, Vx
    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
    //
    // The interpreter takes the decimal value of Vx, and places the hundreds
    // digit in memory at location in I, the tens digit at location I+1, and
    // the ones digit at location I+2.
    fn op_fx33(&mut self, x: usize) {
        let mut value = self.v[x];
        let address: usize = self.index as usize;
        // Ones-place
        self.memory.poke(address + 2, value % 10);
        value /= 10;
        // Tens-place
        self.memory.poke(address + 1, value % 10);
        value /= 10;
        // Hundreds-place
        self.memory.poke(address, value % 10);
    }

    // LD [I], Vx
    // Store registers V0 through Vx in memory starting at location I.
    fn op_fx55(&mut self, x: usize) {
        for i in 0..=x as u16 {
            self.memory.poke((self.index + i) as usize, self.v[i as usize]);
        }
    }

    // LD Vx, [I]
    // Read registers V0 through Vx from memory starting at location I.
    fn op_fx65(&mut self, x: usize) {
        for i in 0..=x as u16 {
            self.v[i as usize] = self.memory.peek((self.index + i) as usize);
        }
    }
}