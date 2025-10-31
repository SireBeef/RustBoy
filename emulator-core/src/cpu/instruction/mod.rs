use std::fmt;

pub enum Instruction {
    ADD(ArithmeticTarget),
    NOP,
    // TODO: Add more instructions as we implement them
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E, // F is the flags register so it is not an ArithmeticTarget
    H,
    L,
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
        }
    }
}
