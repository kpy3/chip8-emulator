const STACK_COUNT: usize = 16;

pub struct Stack {
    stack: [u16; STACK_COUNT],
    stack_pointer: usize
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0; STACK_COUNT],
            stack_pointer: 0
        }
    }

    pub fn push(&mut self, address: u16) {
        self.stack[self.stack_pointer] = address;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }
}