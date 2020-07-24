const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    buffer: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    changed: bool
}

impl Display {
    pub fn new() -> Self {
        Display{
            buffer: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            changed: false
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.changed = true;
    }

    pub fn width(&self) -> usize {
        DISPLAY_WIDTH
    }

    pub fn height(&self) -> usize {
        DISPLAY_HEIGHT
    }

    pub fn peek(&self, x: usize, y: usize) -> u8 {
        self.buffer[y * DISPLAY_WIDTH + x]
    }

    pub fn poke(&mut self, x: usize, y: usize, data: u8) {
        self.buffer[y * DISPLAY_WIDTH + x] = data;
        self.changed = true;
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn draw(&mut self, buf: &mut [u8]) {
        for i in 0..self.buffer.len() {
            if self.buffer[i] == 1 {
                buf[i*4] = 0xff;
                buf[i*4+1] = 0xff;
                buf[i*4+2] = 0xff;
                buf[i*4+3] = 0xff;
            } else {
                buf[i*4] = 0;
                buf[i*4+1] = 0;
                buf[i*4+2] = 0;
                buf[i*4+3] = 0;
            }
        }
        self.changed = false;
    }
}
