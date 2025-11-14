mod flags_register;
use flags_register::FlagsRegister;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0x01,
            b: 0xFF,
            c: 0x13,
            d: 0x00,
            e: 0xC1,
            f: FlagsRegister::new(),
            h: 0x84,
            l: 0x03,
        }
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_bc_register_roundtrip() {
    let mut regs = Registers::new();
    regs.set_bc(0x1234);
    assert_eq!(regs.get_bc(), 0x1234);
    assert_eq!(regs.b, 0x12);
    assert_eq!(regs.c, 0x34);
}

#[test]
fn test_de_register_roundtrip() {
    let mut regs = Registers::new();
    regs.set_de(0xABCD);
    assert_eq!(regs.get_de(), 0xABCD);
    assert_eq!(regs.d, 0xAB);
    assert_eq!(regs.e, 0xCD);
}

#[test]
fn test_hl_register_roundtrip() {
    let mut regs = Registers::new();
    regs.set_hl(0x5678);
    assert_eq!(regs.get_hl(), 0x5678);
    assert_eq!(regs.h, 0x56);
    assert_eq!(regs.l, 0x78);
}

#[test]
fn test_default_is_zero() {
    let regs = Registers::default();
    assert_eq!(regs.a, 0);
    assert_eq!(regs.b, 0);
    assert_eq!(regs.c, 0);
    assert_eq!(regs.d, 0);
    assert_eq!(regs.e, 0);
    // we skip f because f register is for flags
    assert_eq!(regs.h, 0);
    assert_eq!(regs.l, 0);
}
