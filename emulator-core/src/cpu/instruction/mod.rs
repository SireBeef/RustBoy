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
    LD(LoadType),
    ADD(ArithmeticSource),
    CP(ArithmeticSource),
    XOR(ArithmeticSource),
    JP(JumpTest),
}

pub enum DestinationRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum ArithmeticSource {
    A,
    B,
    C,
    D,
    E, // F is the flags register so it is not an ArithmeticSource
    H,
    L,
    HLI,
    D8,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

impl fmt::Display for ArithmeticSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticSource::A => write!(f, "A"),
            ArithmeticSource::B => write!(f, "B"),
            ArithmeticSource::C => write!(f, "C"),
            ArithmeticSource::D => write!(f, "D"),
            ArithmeticSource::E => write!(f, "E"),
            ArithmeticSource::H => write!(f, "H"),
            ArithmeticSource::L => write!(f, "L"),
            ArithmeticSource::HLI => write!(f, "HLI"),
            ArithmeticSource::D8 => write!(f, "D8"),
        }
    }
}
