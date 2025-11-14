pub struct MemoryBus {
    memory: [u8; 0x10000],
    // Later: Add RAM, I/O registers, boot ROM support, etc.
}

impl MemoryBus {
    pub fn new(game_rom: Vec<u8>) -> Self {
        let mut mem = [0; 0x10000];
        mem[..game_rom.len()].copy_from_slice(&game_rom);
        MemoryBus { memory: mem }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
