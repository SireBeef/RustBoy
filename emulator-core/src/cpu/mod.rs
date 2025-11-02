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

    fn decode(&mut self, opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction::NOP,
            0xC3 => Instruction::JP(JumpTest::Always),
            0x81 => Instruction::ADD(ArithmeticTarget::C),
            0xB8 => Instruction::CP(ArithmeticTarget::B),
            0xB9 => Instruction::CP(ArithmeticTarget::C),
            0xBA => Instruction::CP(ArithmeticTarget::D),
            0xBB => Instruction::CP(ArithmeticTarget::E),
            0xBC => Instruction::CP(ArithmeticTarget::H),
            0xBD => Instruction::CP(ArithmeticTarget::L),
            0xBE => Instruction::CP(ArithmeticTarget::HLI),
            0xBF => Instruction::CP(ArithmeticTarget::A),
            0xFE => Instruction::CP(ArithmeticTarget::D8),
            _ => {
                println!(
                    "Unknown opcode: 0x{:02X} at PC: 0x{:04X}",
                    opcode,
                    self.pc - 1
                );
                panic!("Unimplemented opcode!");
            }
        }
    }

    fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
        match instruction {
            Instruction::NOP => (self.pc.wrapping_add(1), 4),
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition)
            }
            Instruction::CP(arithmetic_target) => {
                let (next_counter, cycles) = match arithmetic_target {
                    ArithmeticTarget::HLI => (self.pc.wrapping_add(1), 8),
                    ArithmeticTarget::D8 => (self.pc.wrapping_add(2), 8),
                    _ => (self.pc.wrapping_add(1), 4),
                };
                let val = self.get_value_from_target(arithmetic_target);
                self.compare(val);
                (next_counter, cycles)
            }
            Instruction::ADD(target) => match target {
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    (self.pc.wrapping_add(1), 4)
                }
                _ => {
                    println!("Opcode not implemented!: {}", target);
                    panic!();
                }
            },
        }
    }

    fn get_value_from_target(&self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HLI => self.memory_bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.memory_bus.read_byte(self.pc.wrapping_add(1)),
        }
    }

    fn compare(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        // Half Carry is set if subtracting the lower nibbles of the value with register
        // a will result in a value lower than 0x0.  To avoid underflowing in this test,
        // we can check if the lower nibble of a is less than the lower nibble of the value
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = self.registers.a < value;
    }

    fn jump(&self, should_jump: bool) -> (u16, u8) {
        if should_jump {
            let least_significant_byte = self.memory_bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.memory_bus.read_byte(self.pc + 2) as u16;
            (((most_significant_byte << 8) | least_significant_byte), 16)
        } else {
            (self.pc.wrapping_add(3), 12)
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
    fn manipulate_8bit_register(&mut self, reg: &mut u8, work: impl FnOnce(&mut Cpu, u8) -> u8) {
        let value = *reg;
        *reg = work(self, value);
    }

    pub fn step(&mut self) -> u8 {
        let opcode = self.memory_bus.read_byte(self.pc);
        println!("PC: 0x{:04X}, Opcode: 0x{:02X}", self.pc, opcode);

        let instruction = self.decode(opcode);
        let (next_pc, cycles) = self.execute(instruction);
        println!("Next PC: 0x{:04X}", next_pc);
        self.pc = next_pc;
        cycles
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
