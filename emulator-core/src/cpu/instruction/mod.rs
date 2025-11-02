use std::fmt;

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum Instruction {
    NOP,
    ADD(ArithmeticTarget),
    CP(ArithmeticTarget),
    JP(JumpTest),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E, // F is the flags register so it is not an ArithmeticTarget
    H,
    L,
    HLI,
    D8,
}

impl fmt::Display for ArithmeticTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticTarget::A => write!(f, "A"),
            ArithmeticTarget::B => write!(f, "B"),
            ArithmeticTarget::C => write!(f, "C"),
            ArithmeticTarget::D => write!(f, "D"),
            ArithmeticTarget::E => write!(f, "E"),
            ArithmeticTarget::H => write!(f, "H"),
            ArithmeticTarget::L => write!(f, "L"),
            ArithmeticTarget::HLI => write!(f, "HLI"),
            ArithmeticTarget::D8 => write!(f, "D8"),
        }
    }
}
