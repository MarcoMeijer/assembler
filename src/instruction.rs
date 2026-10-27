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

pub enum Arg {
    Constant(u16),
    Register(Register),
    Ram(Register),
}

pub struct Instruction {
    pub name: String,
    pub args: Vec<Arg>,
}

pub struct Tag(pub String);

pub enum Expr {
    Instruction(Instruction),
    Tag(Tag),
}

pub struct Line {
    pub line_number: usize,
    pub expr: Expr,
}
