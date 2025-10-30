mod instruction;
mod memory_bus;
mod registers;

use instruction::*;
use memory_bus::MemoryBus;
use registers::Registers;

pub struct Cpu {
    registers: Registers,
    memory_bus: MemoryBus,
    pc: u16,
    sp: u16,
}

impl Cpu {
    pub fn new(game_rom: Vec<u8>) -> Self {
        Cpu {
            registers: Registers::new(),
            memory_bus: MemoryBus::new(game_rom),
            // Start at 0x0100 for simplicity (we'll adjust for boot ROM later)
            // We start there because that's where the game code jump instruction starts
            // program counter is used to keep track of where we are in reading the rom
            // at each pc location, we find unsigned 8 bit integers representing
            // op codes -- sometimes instructions are multi-byte (opcode + operands)
            // so eventually we need to increment PC by variable amounts.
            pc: 0x0100,
            sp: 0xFFFE, // Typical initial SP value on Game Boy
                        // 1111 1111 1111 1110
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                _ => {
                    println!("Opcode not implemented!: {}", target);
                    panic!();
                }
            },
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    // TODO make sure this actually works...
    fn manipulate_register(&mut self, reg: &mut u8, work: impl FnOnce(&mut Cpu, u8) -> u8) {
        let value = *reg;
        *reg = work(self, value);
    }

    pub fn step(&mut self) -> u8 {
        1
    }

    pub fn run(&mut self) {
        // This is temporary just to verify we are reading from ROM
        for _ in 0..256 {
            let opcode = self.memory_bus.read_byte(self.pc);
            println!("PC: 0x{:04X}, Opcode: 0x{:02X}", self.pc, opcode);
            self.pc = self.pc.wrapping_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_cpu() -> Cpu {
        Cpu::new(vec![0; 0x8000]) // Dummy ROM of 32KB
    }

    #[test]
    fn test_add_normal() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0x05;
        cpu.registers.c = 0x03;
        let result = cpu.add(cpu.registers.c);
        assert_eq!(result, 0x08, "A + C should equal 0x08");
        assert_eq!(cpu.registers.a, 0x05, "A should not change in add method");
        assert!(!cpu.registers.f.zero, "Zero flag should not be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(!cpu.registers.f.carry, "Carry flag should not be set");
        assert!(
            !cpu.registers.f.half_carry,
            "Half-carry flag should not be set"
        );
    }

    #[test]
    fn test_add_zero_flag() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0x00;
        cpu.registers.c = 0x00;
        let result = cpu.add(cpu.registers.c);
        assert_eq!(result, 0x00, "A + C should equal 0x00");
        assert_eq!(cpu.registers.a, 0x00, "A should not change in add method");
        assert!(cpu.registers.f.zero, "Zero flag should be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(!cpu.registers.f.carry, "Carry flag should not be set");
        assert!(
            !cpu.registers.f.half_carry,
            "Half-carry flag should not be set"
        );
    }

    #[test]
    fn test_add_carry_flag() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0xFF;
        cpu.registers.c = 0x01;
        let result = cpu.add(cpu.registers.c);
        assert_eq!(result, 0x00, "A + C should wrap to 0x00");
        assert_eq!(cpu.registers.a, 0xFF, "A should not change in add method");
        assert!(cpu.registers.f.zero, "Zero flag should be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(cpu.registers.f.carry, "Carry flag should be set");
        assert!(cpu.registers.f.half_carry, "Half-carry flag should be set");
    }

    #[test]
    fn test_add_carry_flag_rollover() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0xFF;
        cpu.registers.c = 0x02;
        let result = cpu.add(cpu.registers.c);
        assert_eq!(result, 0x01, "A + C should wrap to 0x01");
        assert_eq!(cpu.registers.a, 0xFF, "A should not change in add method");
        assert!(!cpu.registers.f.zero, "Zero flag should not be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(cpu.registers.f.carry, "Carry flag should be set");
        assert!(cpu.registers.f.half_carry, "Half-carry flag should be set");
    }

    #[test]
    fn test_add_half_carry_flag() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0x0F;
        cpu.registers.c = 0x01;
        let result = cpu.add(cpu.registers.c);
        assert_eq!(result, 0x10, "A + C should equal 0x10");
        assert_eq!(cpu.registers.a, 0x0F, "A should not change in add method");
        assert!(!cpu.registers.f.zero, "Zero flag should not be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(!cpu.registers.f.carry, "Carry flag should not be set");
        assert!(cpu.registers.f.half_carry, "Half-carry flag should be set");
    }

    #[test]
    fn test_add_combined_flags() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0xFF;
        cpu.registers.c = 0xFF;
        let result = cpu.add(cpu.registers.c);
        assert_eq!(result, 0xFE, "A + C should wrap to 0xFE");
        assert_eq!(cpu.registers.a, 0xFF, "A should not change in add method");
        assert!(!cpu.registers.f.zero, "Zero flag should not be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(cpu.registers.f.carry, "Carry flag should be set");
        assert!(cpu.registers.f.half_carry, "Half-carry flag should be set");
    }
}
