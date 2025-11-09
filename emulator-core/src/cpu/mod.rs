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
            0x01 => Instruction::LD(LoadType::Word(LoadWordTarget::BC)),
            0x05 => Instruction::DEC(ArithmeticSource::B),
            0x06 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8)),
            0x0d => Instruction::DEC(ArithmeticSource::C),
            0x0e => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8)),
            0x11 => Instruction::LD(LoadType::Word(LoadWordTarget::DE)),
            0x15 => Instruction::DEC(ArithmeticSource::D),
            0x16 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8)),
            0x18 => Instruction::JR(JumpTest::Always),
            0x1d => Instruction::DEC(ArithmeticSource::E),
            0x1e => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8)),
            0x20 => Instruction::JR(JumpTest::NotZero),
            0x21 => Instruction::LD(LoadType::Word(LoadWordTarget::HL)),
            0x25 => Instruction::DEC(ArithmeticSource::H),
            0x26 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8)),
            0x28 => Instruction::JR(JumpTest::Zero),
            0x2d => Instruction::DEC(ArithmeticSource::L),
            0x2e => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8)),
            0x31 => Instruction::LD(LoadType::Word(LoadWordTarget::SP)),
            0x30 => Instruction::JR(JumpTest::NotCarry),
            0x32 => Instruction::LD(LoadType::IndirectFromA(Indirect::HLIndirectMinus)),
            0x35 => Instruction::DEC(ArithmeticSource::HLI),
            0x36 => Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D8)),
            0x38 => Instruction::JR(JumpTest::Carry),
            0x3d => Instruction::DEC(ArithmeticSource::A),
            0x3e => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8)),

            0x40 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B)),
            0x41 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C)),
            0x42 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D)),
            0x43 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E)),
            0x44 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H)),
            0x45 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L)),
            0x46 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HLI)),
            0x47 => Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A)),
            0x48 => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B)),
            0x49 => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C)),
            0x4a => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D)),
            0x4b => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E)),
            0x4c => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H)),
            0x4d => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L)),
            0x4e => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI)),
            0x4f => Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A)),

            0x50 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B)),
            0x51 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C)),
            0x52 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D)),
            0x53 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E)),
            0x54 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H)),
            0x55 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L)),
            0x56 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLI)),
            0x57 => Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A)),
            0x58 => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B)),
            0x59 => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C)),
            0x5a => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D)),
            0x5b => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E)),
            0x5c => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H)),
            0x5d => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L)),
            0x5e => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLI)),
            0x5f => Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A)),

            0x60 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B)),
            0x61 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C)),
            0x62 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D)),
            0x63 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E)),
            0x64 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H)),
            0x65 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L)),
            0x66 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLI)),
            0x67 => Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A)),
            0x68 => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B)),
            0x69 => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C)),
            0x6a => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D)),
            0x6b => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E)),
            0x6c => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H)),
            0x6d => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L)),
            0x6e => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLI)),
            0x6f => Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A)),

            0x70 => Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::B)),
            0x71 => Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::C)),
            0x72 => Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D)),
            0x73 => Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::E)),
            0x74 => Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::H)),
            0x75 => Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::L)),

            0x77 => Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A)),
            0x78 => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B)),
            0x79 => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C)),
            0x7a => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D)),
            0x7b => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E)),
            0x7c => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H)),
            0x7d => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L)),
            0x7e => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI)),
            0x7f => Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A)),

            0x81 => Instruction::ADD(ArithmeticSource::C),
            0xA8 => Instruction::XOR(ArithmeticSource::B),
            0xA9 => Instruction::XOR(ArithmeticSource::C),
            0xAA => Instruction::XOR(ArithmeticSource::D),
            0xAB => Instruction::XOR(ArithmeticSource::E),
            0xAC => Instruction::XOR(ArithmeticSource::H),
            0xAD => Instruction::XOR(ArithmeticSource::L),
            0xAE => Instruction::XOR(ArithmeticSource::HLI),
            0xAF => Instruction::XOR(ArithmeticSource::A),
            0xC3 => Instruction::JP(JumpTest::Always),
            0xB8 => Instruction::CP(ArithmeticSource::B),
            0xB9 => Instruction::CP(ArithmeticSource::C),
            0xBA => Instruction::CP(ArithmeticSource::D),
            0xBB => Instruction::CP(ArithmeticSource::E),
            0xBC => Instruction::CP(ArithmeticSource::H),
            0xBD => Instruction::CP(ArithmeticSource::L),
            0xBE => Instruction::CP(ArithmeticSource::HLI),
            0xBF => Instruction::CP(ArithmeticSource::A),
            0xEE => Instruction::XOR(ArithmeticSource::D8),
            0xFE => Instruction::CP(ArithmeticSource::D8),
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
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::B => self.registers.b,
                        LoadByteSource::C => self.registers.c,
                        LoadByteSource::D => self.registers.d,
                        LoadByteSource::E => self.registers.e,
                        LoadByteSource::H => self.registers.h,
                        LoadByteSource::L => self.registers.l,
                        LoadByteSource::D8 => self.memory_bus.read_byte(self.pc.wrapping_add(1)),
                        LoadByteSource::HLI => self.memory_bus.read_byte(self.registers.get_hl()),
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::B => self.registers.b = source_value,
                        LoadByteTarget::C => self.registers.c = source_value,
                        LoadByteTarget::D => self.registers.d = source_value,
                        LoadByteTarget::E => self.registers.e = source_value,
                        LoadByteTarget::H => self.registers.h = source_value,
                        LoadByteTarget::L => self.registers.l = source_value,
                        LoadByteTarget::HLI => self
                            .memory_bus
                            .write_byte(self.registers.get_hl(), source_value),
                    };
                    match source {
                        LoadByteSource::D8 => (self.pc.wrapping_add(2), 8),
                        LoadByteSource::HLI => (self.pc.wrapping_add(1), 8),
                        _ => (self.pc.wrapping_add(1), 4),
                    }
                }
                LoadType::Word(target) => {
                    let word = self.read_next_word();
                    match target {
                        LoadWordTarget::BC => self.registers.set_bc(word),
                        LoadWordTarget::DE => self.registers.set_de(word),
                        LoadWordTarget::HL => self.registers.set_hl(word),
                        LoadWordTarget::SP => self.sp = word,
                    };
                    (self.pc.wrapping_add(3), 12)
                }
                LoadType::IndirectFromA(target) => {
                    let a = self.registers.a;
                    match target {
                        Indirect::BCIndirect => {
                            let bc = self.registers.get_bc();
                            self.memory_bus.write_byte(bc, a)
                        }
                        Indirect::DEIndirect => {
                            let de = self.registers.get_de();
                            self.memory_bus.write_byte(de, a)
                        }
                        Indirect::HLIndirectMinus => {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_sub(1));
                            self.memory_bus.write_byte(hl, a);
                        }
                        Indirect::HLIndirectPlus => {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_add(1));
                            self.memory_bus.write_byte(hl, a);
                        }
                        Indirect::WordIndirect => {
                            let word = self.read_next_word();
                            self.memory_bus.write_byte(word, a);
                        }
                        Indirect::LastByteIndirect => {
                            let c = self.registers.c as u16;
                            self.memory_bus.write_byte(0xFF00 + c, a);
                        }
                    };

                    match target {
                        Indirect::WordIndirect => (self.pc.wrapping_add(3), 16),
                        _ => (self.pc.wrapping_add(1), 8),
                    }
                }
            },
            Instruction::ADD(source) => match source {
                ArithmeticSource::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    (self.pc.wrapping_add(1), 4)
                }
                _ => {
                    println!("Opcode not implemented!: {}", source);
                    panic!();
                }
            },
            Instruction::DEC(target) => match target {
                ArithmeticSource::HLI => {
                    let value = self.memory_bus.read_byte(self.registers.get_hl());
                    let new_val = self.dec(value);
                    self.memory_bus.write_byte(self.registers.get_hl(), new_val);
                    (self.pc.wrapping_add(1), 12)
                }
                _ => {
                    self.manipulate_8bit_register(
                        target.to_destination_register(),
                        self.get_value_from_source(target),
                        Cpu::dec,
                    );
                    (self.pc.wrapping_add(1), 4)
                }
            },
            Instruction::XOR(source) => {
                let (next_counter, cycles) = match source {
                    ArithmeticSource::HLI => (self.pc.wrapping_add(1), 8),
                    ArithmeticSource::D8 => (self.pc.wrapping_add(2), 8),
                    _ => (self.pc.wrapping_add(1), 4),
                };
                let val = self.get_value_from_source(source);
                self.manipulate_8bit_register(DestinationRegister::A, val, Cpu::xor);
                (next_counter, cycles)
            }
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
            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump_relative(jump_condition)
            }
            Instruction::CP(source) => {
                let (next_counter, cycles) = match source {
                    ArithmeticSource::HLI => (self.pc.wrapping_add(1), 8),
                    ArithmeticSource::D8 => (self.pc.wrapping_add(2), 8),
                    _ => (self.pc.wrapping_add(1), 4),
                };
                let val = self.get_value_from_source(source);
                self.compare(val);
                (next_counter, cycles)
            }
        }
    }

    fn read_next_word(&self) -> u16 {
        let least_significant_byte = self.memory_bus.read_byte(self.pc.wrapping_add(1)) as u16;
        let most_significant_byte = self.memory_bus.read_byte(self.pc.wrapping_add(2)) as u16;
        ((most_significant_byte << 8) | least_significant_byte)
    }

    fn get_value_from_source(&self, source: ArithmeticSource) -> u8 {
        match source {
            ArithmeticSource::A => self.registers.a,
            ArithmeticSource::B => self.registers.b,
            ArithmeticSource::C => self.registers.c,
            ArithmeticSource::D => self.registers.d,
            ArithmeticSource::E => self.registers.e,
            ArithmeticSource::H => self.registers.h,
            ArithmeticSource::L => self.registers.l,
            ArithmeticSource::HLI => self.memory_bus.read_byte(self.registers.get_hl()),
            ArithmeticSource::D8 => self.memory_bus.read_byte(self.pc.wrapping_add(1)),
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
            let least_significant_byte = self.memory_bus.read_byte(self.pc.wrapping_add(1)) as u16;
            let most_significant_byte = self.memory_bus.read_byte(self.pc.wrapping_add(2)) as u16;
            (((most_significant_byte << 8) | least_significant_byte), 16)
        } else {
            (self.pc.wrapping_add(3), 12)
        }
    }

    fn jump_relative(&self, should_jump: bool) -> (u16, u8) {
        let next_step = self.pc.wrapping_add(2);
        if should_jump {
            let offset = self.memory_bus.read_byte(self.pc + 1) as i8;
            let pc = if offset >= 0 {
                next_step.wrapping_add(offset as u16)
            } else {
                next_step.wrapping_sub(offset.abs() as u16)
            };
            (pc, 12)
        } else {
            (next_step, 8)
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

    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) == 0x0;
        new_value
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    fn manipulate_8bit_register(
        &mut self,
        destination_register: DestinationRegister,
        value: u8,
        work: impl FnOnce(&mut Cpu, u8) -> u8,
    ) {
        match destination_register {
            DestinationRegister::A => self.registers.a = work(self, value),
            DestinationRegister::B => self.registers.b = work(self, value),
            DestinationRegister::C => self.registers.c = work(self, value),
            DestinationRegister::D => self.registers.d = work(self, value),
            DestinationRegister::E => self.registers.e = work(self, value),
            DestinationRegister::H => self.registers.h = work(self, value),
            DestinationRegister::L => self.registers.l = work(self, value),
            _ => panic!("Unknown destination register"),
        };
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

    #[test]
    fn test_xor_normal() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0b11110000;
        let value = 0b10101010;
        let result = cpu.xor(value);
        assert_eq!(result, 0b01011010, "XOR should produce correct result");
        assert!(!cpu.registers.f.zero, "Zero flag should not be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(
            !cpu.registers.f.half_carry,
            "Half-carry flag should not be set"
        );
        assert!(!cpu.registers.f.carry, "Carry flag should not be set");
    }

    #[test]
    fn test_xor_zero_flag() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0b10101010;
        let value = 0b10101010;
        let result = cpu.xor(value);
        assert_eq!(result, 0x00, "XOR with same value should produce 0");
        assert!(cpu.registers.f.zero, "Zero flag should be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(
            !cpu.registers.f.half_carry,
            "Half-carry flag should not be set"
        );
        assert!(!cpu.registers.f.carry, "Carry flag should not be set");
    }

    #[test]
    fn test_xor_with_zero() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0xFF;
        let value = 0x00;
        let result = cpu.xor(value);
        assert_eq!(result, 0xFF, "XOR with 0 should return original value");
        assert!(!cpu.registers.f.zero, "Zero flag should not be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(
            !cpu.registers.f.half_carry,
            "Half-carry flag should not be set"
        );
        assert!(!cpu.registers.f.carry, "Carry flag should not be set");
    }

    #[test]
    fn test_xor_self() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0xA5;
        let result = cpu.xor(cpu.registers.a);
        assert_eq!(result, 0x00, "XOR with self should always produce 0");
        assert!(cpu.registers.f.zero, "Zero flag should be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(
            !cpu.registers.f.half_carry,
            "Half-carry flag should not be set"
        );
        assert!(!cpu.registers.f.carry, "Carry flag should not be set");
    }

    #[test]
    fn test_xor_all_bits() {
        let mut cpu = setup_cpu();
        cpu.registers.a = 0xFF;
        let value = 0xFF;
        let result = cpu.xor(value);
        assert_eq!(result, 0x00, "XOR 0xFF with 0xFF should produce 0");
        assert!(cpu.registers.f.zero, "Zero flag should be set");
        assert!(!cpu.registers.f.subtract, "Subtract flag should not be set");
        assert!(
            !cpu.registers.f.half_carry,
            "Half-carry flag should not be set"
        );
        assert!(!cpu.registers.f.carry, "Carry flag should not be set");
    }

    #[test]
    fn test_xor_clears_flags() {
        let mut cpu = setup_cpu();
        // Set all flags first
        cpu.registers.f.zero = true;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = true;

        cpu.registers.a = 0b11110000;
        let value = 0b00001111;
        let result = cpu.xor(value);
        assert_eq!(result, 0xFF, "XOR should produce correct result");
        assert!(!cpu.registers.f.zero, "Zero flag should be cleared");
        assert!(!cpu.registers.f.subtract, "Subtract flag should be cleared");
        assert!(
            !cpu.registers.f.half_carry,
            "Half-carry flag should be cleared"
        );
        assert!(!cpu.registers.f.carry, "Carry flag should be cleared");
    }
}
