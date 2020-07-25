mod font;
mod vm;
mod keypad;
mod stack;
mod display;
mod memory;

use std::env;

mod chip8;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = std::fs::read(filename).map_err(|e| format!("{}", e))?;
    vm::run(&data);
    Ok(())
}

