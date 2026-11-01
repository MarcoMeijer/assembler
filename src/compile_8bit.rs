use crate::{Arg, Expr, Instruction, Line, Register, expect_arg_count};

const MOVE: u8 = 0b10000000;

const BITWISE_OR: u8 = 0b01000000;
const BITWISE_ADD: u8 = 0b01000001;
const BITWISE_SUB: u8 = 0b01000010;
const BITWISE_NAND: u8 = 0b01000011;
const BITWISE_AND: u8 = 0b01000100;
const BITWISE_NOT: u8 = 0b01000101;
const BITWISE_XOR: u8 = 0b01000110;
const BITWISE_EQ: u8 = 0b01000111;

const READ_RAM_U8: u8 = 0b11000000;
const WRITE_RAM_U8: u8 = 0b11000001;

const ARG_A_POS: usize = 3;

type InstructionCompiler = fn(&Instruction) -> Result<u8, String>;

pub fn compile_lines_8bit(lines: &[Line]) -> Option<Vec<u8>> {
    let mut result = vec![];
    let mut error = false;

    for line in lines {
        let Line { line_number, expr } = line;

        match expr {
            Expr::Tag(_) => {
                continue;
            }
            Expr::Instruction(instruction) => {
                let compiler: InstructionCompiler = match instruction.name.as_str() {
                    "mov" => compile_mov,
                    "or" => compile_or,
                    "add" => compile_add,
                    "sub" => compile_sub,
                    "nand" => compile_nand,
                    "and" => compile_and,
                    "not" => compile_not,
                    "xor" => compile_xor,
                    "eq" => compile_eq,
                    x => {
                        println!("Unknown instruction on line {line_number}: '{x}'");
                        error = true;
                        continue;
                    }
                };

                let compile_result = compiler(instruction);

                match compile_result {
                    Ok(res) => {
                        result.push(res);
                    }
                    Err(err) => {
                        println!("Compile error on line {line_number}: '{err}'");
                        error = true;
                        continue;
                    }
                }
            }
        }
    }

    if error { None } else { Some(result) }
}

fn compile_mov(instruction: &Instruction) -> Result<u8, String> {
    expect_arg_count(instruction, 2)?;
    let arg1 = &instruction.args[0];
    let arg2 = &instruction.args[1];

    match (arg1, arg2) {
        (Arg::Constant(_), _) => return Err("Can't move into a constant".into()),
        (Arg::Tag(_), _) => return Err("Can't move into a tag".into()),
        (_, Arg::Tag(_)) => return Err("Can't move a tag into constant".into()),
        (Arg::Ram(_), Arg::Constant(_)) => {
            return Err("Can't move constant directly into ram".into());
        }
        (Arg::Ram(_), Arg::Ram(_)) => {
            return Err("Can't move ram value directly into ram".into());
        }
        (Arg::Register(r), Arg::Constant(x)) => {
            if *r != Register::Reg1 {
                return Err("Can only move constant into register 1".into());
            }
            Ok(*x as u8)
        }
        (Arg::Register(dest), Arg::Register(a)) => Ok(MOVE | dest.u8() | a.u8() << ARG_A_POS),
        (Arg::Register(register), Arg::Ram(ram)) => {
            if *register != Register::Reg1 {
                return Err("Can only move ram value into register 1".into());
            }
            if *ram != Register::Reg2 {
                return Err("Can only use register 2 as ram address".into());
            }
            Ok(READ_RAM_U8)
        }
        (Arg::Ram(ram), Arg::Register(register)) => {
            if *register != Register::Reg1 {
                return Err("Can only write register 1 to ram".into());
            }
            if *ram != Register::Reg2 {
                return Err("Can only use register 2 as ram address".into());
            }
            Ok(WRITE_RAM_U8)
        }
    }
}

fn compile_or(instruction: &Instruction) -> Result<u8, String> {
    compile_operation(instruction, BITWISE_OR)
}

fn compile_nand(instruction: &Instruction) -> Result<u8, String> {
    compile_operation(instruction, BITWISE_NAND)
}

fn compile_and(instruction: &Instruction) -> Result<u8, String> {
    compile_operation(instruction, BITWISE_AND)
}

fn compile_not(instruction: &Instruction) -> Result<u8, String> {
    compile_single_arg_operation(instruction, BITWISE_NOT)
}

fn compile_add(instruction: &Instruction) -> Result<u8, String> {
    compile_operation(instruction, BITWISE_ADD)
}

fn compile_sub(instruction: &Instruction) -> Result<u8, String> {
    compile_operation(instruction, BITWISE_SUB)
}

fn compile_xor(instruction: &Instruction) -> Result<u8, String> {
    compile_operation(instruction, BITWISE_XOR)
}

fn compile_eq(instruction: &Instruction) -> Result<u8, String> {
    compile_operation(instruction, BITWISE_EQ)
}

fn compile_operation(instruction: &Instruction, opcode: u8) -> Result<u8, String> {
    expect_arg_count(instruction, 3)?;
    let arg1 = &instruction.args[0];
    let arg2 = &instruction.args[1];
    let arg3 = &instruction.args[2];

    match (arg1, arg2, arg3) {
        (Arg::Register(dest), Arg::Register(a), Arg::Register(b)) => {
            if *a != Register::Reg1 {
                return Err("Argument a should be register 1".into());
            }
            if *b != Register::Reg2 {
                return Err("Argument b should be register 2".into());
            }
            Ok(opcode | dest.u8() << ARG_A_POS)
        }
        _ => {
            return Err("Can only do this operation on registers".into());
        }
    }
}

fn compile_single_arg_operation(instruction: &Instruction, opcode: u8) -> Result<u8, String> {
    expect_arg_count(instruction, 2)?;
    let arg1 = &instruction.args[0];
    let arg2 = &instruction.args[1];

    match (arg1, arg2) {
        (Arg::Register(dest), Arg::Register(a)) => {
            if *a != Register::Reg1 {
                return Err("Argument a should be register 1".into());
            }
            Ok(opcode | dest.u8() << ARG_A_POS)
        }
        _ => {
            return Err("Can only do this operation on registers".into());
        }
    }
}
