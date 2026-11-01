#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Register {
    Zero,
    TX,
    AX,
    BX,
    CX,
    DX,
    RIP,
    RSP,
}

impl Register {
    pub fn u8(&self) -> u8 {
        match self {
            Register::Zero => 0,
            Register::TX => 1,
            Register::AX => 2,
            Register::BX => 3,
            Register::CX => 4,
            Register::DX => 5,
            Register::RIP => 6,
            Register::RSP => 7,
        }
    }

    pub fn u16(&self) -> u16 {
        self.u8() as u16
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Arg {
    Constant(u16),
    Register(Register),
    Ram(Register),
    Tag(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instruction {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tag(pub String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    Instruction(Instruction),
    Tag(Tag),
}

pub struct Line {
    pub line_number: usize,
    pub expr: Expr,
}
