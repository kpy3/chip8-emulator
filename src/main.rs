mod font;
mod vm;
mod keypad;
mod stack;
mod display;
mod memory;
mod chip8;

use clap::clap_app;
use std::io;

fn main() -> io::Result<()> {
    let matches = clap_app!(chip8 =>
        (about: "Chip-8 Emulator")
        (version: "0.1")
        (@arg SLEEP_DURATION: -s --sleep +takes_value "Sets a timeout between ticks in milliseconds, default 2")
        (@arg ROM: +required "Path to ROM to load")
    ).get_matches();

    let sleep_duration: u64 = matches.value_of_t("SLEEP_DURATION").unwrap_or(2);
    let filename = matches
        .value_of("ROM")
        .expect("No ROM filename set");

    let data = std::fs::read(filename)?;
    vm::run(&data, sleep_duration);
    Ok(())
}

