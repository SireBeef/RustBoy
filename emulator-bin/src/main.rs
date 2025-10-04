use std::fs::File;
use std::io::Read;

use emulator_core::Cpu;

fn main() {
    let mut f = File::open("roms/pokemon-red.gb").unwrap();
    let mut rom = Vec::new();
    let _ = f.read_to_end(&mut rom);
    let mut cpu = Cpu::new(rom);

    cpu.run();
}
