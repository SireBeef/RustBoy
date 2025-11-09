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
    DEC(ArithmeticSource),
    CP(ArithmeticSource),
    XOR(ArithmeticSource),
    JP(JumpTest),
    JR(JumpTest),
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

#[derive(Debug)]
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

pub enum Indirect {
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
    IndirectFromA(Indirect),
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

impl ArithmeticSource {
    pub fn to_destination_register(&self) -> DestinationRegister {
        match self {
            ArithmeticSource::A => DestinationRegister::A,
            ArithmeticSource::B => DestinationRegister::B,
            ArithmeticSource::C => DestinationRegister::C,
            ArithmeticSource::D => DestinationRegister::D,
            ArithmeticSource::E => DestinationRegister::E,
            ArithmeticSource::H => DestinationRegister::H,
            ArithmeticSource::L => DestinationRegister::L,
            ArithmeticSource::HLI | ArithmeticSource::D8 => {
                panic!("Cannot convert {:?} to DestinationRegister", self)
            }
        }
    }
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
