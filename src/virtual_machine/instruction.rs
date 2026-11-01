use super::{CMP_REGISTER, PROGRAM_COUNTER, Register};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Constant(u16),
    ALU(Opcode),
    IO(IO),
    Jump(Jump),
    Noop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    Or,
    Nand,
    And,
    Nor,
    Add,
    Sub,
    Cmp,
    Noop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO {
    ReadRamU8,
    ReadRamU16,
    WriteRamU8,
    WriteRamU16,
    Output,
    Noop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jump {
    Je,
    Jne,
    Jg,
    Jng,
    Jl,
    Jnl,
    Jmp,
    Noop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub mode: Mode,
    pub destination: Register,
    pub argument_a: Register,
    pub argument_b: Register,
}

pub const MODE_BITS: u16 = 0b11000000_00000000;
pub const ARGUMENT_A_BITS: u16 = 0b00111000_00000000;
pub const ARGUMENT_B_BITS: u16 = 0b00000111_00000000;
pub const DESTINATION_BITS: u16 = 0b00000000_11100000;
pub const OPCODE_BITS: u16 = 0b00000000_00001111;

fn decode_opcode(opcode: u16) -> Opcode {
    match opcode {
        0b0000 => Opcode::Or,
        0b0001 => Opcode::Nand,
        0b0010 => Opcode::And,
        0b0011 => Opcode::Nor,
        0b0100 => Opcode::Add,
        0b0101 => Opcode::Sub,
        0b0110 => Opcode::Cmp,
        _ => Opcode::Noop,
    }
}

fn decode_io(io: u16) -> IO {
    match io {
        0b0000 => IO::ReadRamU8,
        0b0001 => IO::ReadRamU16,
        0b0010 => IO::WriteRamU8,
        0b0011 => IO::WriteRamU16,
        0b0100 => IO::Output,
        _ => IO::Noop,
    }
}

fn decode_jump(jump: u16) -> Jump {
    match jump {
        0b0000 => Jump::Je,
        0b0001 => Jump::Jne,
        0b0010 => Jump::Jg,
        0b0011 => Jump::Jng,
        0b0100 => Jump::Jl,
        0b0101 => Jump::Jnl,
        0b0110 => Jump::Jmp,
        _ => Jump::Noop,
    }
}

fn decode_mode(mode: u16, instruction: u16) -> Mode {
    match mode {
        0b00 => Mode::Constant(instruction),
        0b01 => Mode::ALU(decode_opcode(instruction & OPCODE_BITS)),
        0b10 => Mode::IO(decode_io(instruction & OPCODE_BITS)),
        0b11 => Mode::Jump(decode_jump(instruction & OPCODE_BITS)),
        _ => Mode::Noop,
    }
}

pub fn decode_instruction(instruction: u16) -> Instruction {
    let mode = decode_mode((instruction & MODE_BITS) >> 14, instruction);
    let mut destination = ((instruction & DESTINATION_BITS) >> 5) as Register;
    let argument_a = ((instruction & ARGUMENT_A_BITS) >> 11) as Register;
    let argument_b = ((instruction & ARGUMENT_B_BITS) >> 8) as Register;

    match mode {
        Mode::Constant(_) => {
            destination = 1;
        }
        Mode::ALU(Opcode::Cmp) => {
            destination = CMP_REGISTER;
        }
        Mode::IO(IO::WriteRamU8 | IO::WriteRamU16 | IO::Output | IO::Noop) => {
            destination = 0;
        }
        Mode::Jump(_) => {
            destination = PROGRAM_COUNTER;
        }
        _ => {}
    }

    Instruction {
        mode,
        destination,
        argument_a,
        argument_b,
    }
}
