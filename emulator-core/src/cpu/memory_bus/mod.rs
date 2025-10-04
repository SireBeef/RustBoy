pub struct MemoryBus {
    rom: Vec<u8>,
    // Later: Add RAM, I/O registers, boot ROM support, etc.
}

impl MemoryBus {
    pub fn new(game_rom: Vec<u8>) -> Self {
        MemoryBus { rom: game_rom }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    // pub fn write_byte(&mut self, _address: u16, _value: u8) {
    //     // Stub for now; implement when needed for RAM/I/O
    // }
}
