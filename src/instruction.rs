#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Register {
    TX,
    AX,
    BX,
    CX,
    DX,
    RBP,
    RSP,
    RIP,
}

impl Register {
    pub fn u8(&self) -> u8 {
        match self {
            Register::TX => 0,
            Register::AX => 1,
            Register::BX => 2,
            Register::CX => 3,
            Register::DX => 4,
            Register::RBP => 5,
            Register::RSP => 6,
            Register::RIP => 7,
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
