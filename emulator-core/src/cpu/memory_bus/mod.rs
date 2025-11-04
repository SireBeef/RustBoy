pub struct MemoryBus {
    rom: Vec<u8>,
    memory: [u8; 0xFFFF],
    // Later: Add RAM, I/O registers, boot ROM support, etc.
}

impl MemoryBus {
    pub fn new(game_rom: Vec<u8>) -> Self {
        MemoryBus {
            rom: game_rom,
            memory: [0; 0xFFFF],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
