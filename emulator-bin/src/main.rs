mod rom_loader;

use emulator_core::Cpu;
use rom_loader::load_rom;
use std::process;

fn main() {
    let rom_path = "roms/pokemon-blue.gb";

    let rom = match load_rom(rom_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading ROM: {}", e);
            eprintln!("Please ensure the ROM file exists at: {}", rom_path);
            process::exit(1);
        }
    };

    let mut cpu = Cpu::new(rom);
    cpu.run();
}
