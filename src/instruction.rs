#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Register {
    Zero,
    Reg1,
    Reg2,
    Reg3,
    Reg4,
    Reg5,
    Reg6,
    RSP,
}

impl Register {
    pub fn u16(&self) -> u16 {
        match self {
            Register::Zero => 0,
            Register::Reg1 => 1,
            Register::Reg2 => 2,
            Register::Reg3 => 3,
            Register::Reg4 => 4,
            Register::Reg5 => 5,
            Register::Reg6 => 6,
            Register::RSP => 7,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arg {
    Constant(u16),
    Register(Register),
    Ram(Register),
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
