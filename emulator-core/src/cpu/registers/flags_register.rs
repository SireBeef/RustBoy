#[derive(Clone, Copy, Debug)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagsRegister {
    pub fn new() -> Self {
        Self {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[test]
fn test_zero_flag() {
    let flags = FlagsRegister {
        zero: true,
        subtract: false,
        half_carry: false,
        carry: false,
    };
    let byte: u8 = flags.into();
    assert_eq!(byte, 0b1000_0000);
    let restored: FlagsRegister = byte.into();
    assert!(restored.zero);
    assert!(!restored.subtract);
    assert!(!restored.half_carry);
    assert!(!restored.carry);
}

#[test]
fn test_subtract_flag() {
    let flags = FlagsRegister {
        zero: false,
        subtract: true,
        half_carry: false,
        carry: false,
    };
    let byte: u8 = flags.into();
    assert_eq!(byte, 0b0100_0000);
    let restored: FlagsRegister = byte.into();
    assert!(!restored.zero);
    assert!(restored.subtract);
    assert!(!restored.half_carry);
    assert!(!restored.carry);
}

#[test]
fn test_half_carry_flag() {
    let flags = FlagsRegister {
        zero: false,
        subtract: false,
        half_carry: true,
        carry: false,
    };
    let byte: u8 = flags.into();
    assert_eq!(byte, 0b0010_0000);
    let restored: FlagsRegister = byte.into();
    assert!(!restored.zero);
    assert!(!restored.subtract);
    assert!(restored.half_carry);
    assert!(!restored.carry);
}

#[test]
fn test_carry_flag() {
    let flags = FlagsRegister {
        zero: false,
        subtract: false,
        half_carry: false,
        carry: true,
    };
    let byte: u8 = flags.into();
    assert_eq!(byte, 0b0001_0000);
    let restored: FlagsRegister = byte.into();
    assert!(!restored.zero);
    assert!(!restored.subtract);
    assert!(!restored.half_carry);
    assert!(restored.carry);
}

#[test]
fn test_all_flags_set() {
    let flags = FlagsRegister {
        zero: true,
        subtract: true,
        half_carry: true,
        carry: true,
    };
    let byte: u8 = flags.into();
    assert_eq!(byte, 0b1111_0000); // All flags at bits 7–4
    let restored: FlagsRegister = byte.into();
    assert!(restored.zero);
    assert!(restored.subtract);
    assert!(restored.half_carry);
    assert!(restored.carry);
}

#[test]
fn test_no_flags_set() {
    let flags = FlagsRegister {
        zero: false,
        subtract: false,
        half_carry: false,
        carry: false,
    };
    let byte: u8 = flags.into();
    assert_eq!(byte, 0b0000_0000); // No flags set
    let restored: FlagsRegister = byte.into();
    assert!(!restored.zero);
    assert!(!restored.subtract);
    assert!(!restored.half_carry);
    assert!(!restored.carry);
}

#[test]
fn test_partial_flags() {
    let flags = FlagsRegister {
        zero: true,
        subtract: false,
        half_carry: true,
        carry: false,
    };
    let byte: u8 = flags.into();
    assert_eq!(byte, 0b1010_0000); // Zero and half-carry flags set
    let restored: FlagsRegister = byte.into();
    assert!(restored.zero);
    assert!(!restored.subtract);
    assert!(restored.half_carry);
    assert!(!restored.carry);
}

#[test]
fn test_roundtrip_conversion() {
    let original = FlagsRegister {
        zero: true,
        subtract: false,
        half_carry: true,
        carry: true,
    };
    let byte: u8 = original.into();
    let restored: FlagsRegister = byte.into();
    assert_eq!(restored.zero, original.zero);
    assert_eq!(restored.subtract, original.subtract);
    assert_eq!(restored.half_carry, original.half_carry);
    assert_eq!(restored.carry, original.carry);
}

#[test]
fn test_unused_bits_ignored() {
    let byte: u8 = 0b1111_1111; // Set all bits, including unused ones
    let flags: FlagsRegister = byte.into();
    assert!(flags.zero);
    assert!(flags.subtract);
    assert!(flags.half_carry);
    assert!(flags.carry);
    let byte_back: u8 = flags.into();
    assert_eq!(byte_back, 0b1111_0000);
}

#[test]
fn test_arbitrary_byte_input() {
    let byte: u8 = 0b1010_1010;
    let flags: FlagsRegister = byte.into();
    assert!(flags.zero);
    assert!(!flags.subtract);
    assert!(flags.half_carry);
    assert!(!flags.carry);
}
