use crate::{Arg, Expr, Instruction, Line, Register};

const ALU: u16 = 0b01000000_00000000;
const IO: u16 = 0b10000000_00000000;

const BITWISE_OR: u16 = 0b01000000_00000000;
const BITWISE_NAND: u16 = 0b01000000_00000001;
const BITWISE_AND: u16 = 0b01000000_00000010;
const BITWISE_NOR: u16 = 0b01000000_00000011;
const BITWISE_ADD: u16 = 0b01000000_00000100;
const BITWISE_SUB: u16 = 0b01000000_00000101;

const READ_RAM_U8: u16 = 0b0000;
const READ_RAM_U16: u16 = 0b0001;
const WRITE_RAM_U8: u16 = 0b0010;
const WRITE_RAM_U16: u16 = 0b0011;
const OUTPUT: u16 = 0b0100;

const ARG_A_POS: usize = 11;
const ARG_B_POS: usize = 7;
const DEST_POS: usize = 4;

pub type InstructionCompiler = fn(&Instruction) -> Result<u16, String>;

pub fn compile_lines(lines: &[Line]) -> Option<Vec<u16>> {
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
                    "nand" => compile_nand,
                    "and" => compile_and,
                    "nor" => compile_nor,
                    "add" => compile_add,
                    "sub" => compile_sub,
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

fn compile_mov(instruction: &Instruction) -> Result<u16, String> {
    expect_arg_count(instruction, 2)?;
    let arg1 = instruction.args[0];
    let arg2 = instruction.args[1];

    match (arg1, arg2) {
        (Arg::Constant(_), _) => return Err("Can't move into a constant".into()),
        (Arg::Ram(_), Arg::Constant(_)) => {
            return Err("Can't move constant directly into ram".into());
        }
        (Arg::Ram(_), Arg::Ram(_)) => {
            return Err("Can't move ram value directly into ram".into());
        }
        (Arg::Register(r), Arg::Constant(x)) => {
            if r != Register::Reg1 {
                return Err("Can only move constant into register 1".into());
            }
            Ok(x)
        }
        (Arg::Register(dest), Arg::Register(a)) => {
            Ok(ALU | dest.u16() << DEST_POS | a.u16() << ARG_A_POS)
        }
        (Arg::Register(register), Arg::Ram(ram)) => {
            Ok(IO | READ_RAM_U16 | register.u16() << DEST_POS | ram.u16() << ARG_A_POS)
        }
        (Arg::Ram(ram), Arg::Register(register)) => {
            Ok(IO | WRITE_RAM_U16 | ram.u16() << ARG_A_POS | register.u16() << ARG_B_POS)
        }
    }
}

fn compile_or(instruction: &Instruction) -> Result<u16, String> {
    compile_operation(instruction, BITWISE_OR)
}

fn compile_nand(instruction: &Instruction) -> Result<u16, String> {
    compile_operation(instruction, BITWISE_NAND)
}

fn compile_and(instruction: &Instruction) -> Result<u16, String> {
    compile_operation(instruction, BITWISE_AND)
}

fn compile_nor(instruction: &Instruction) -> Result<u16, String> {
    compile_operation(instruction, BITWISE_NOR)
}

fn compile_add(instruction: &Instruction) -> Result<u16, String> {
    compile_operation(instruction, BITWISE_ADD)
}

fn compile_sub(instruction: &Instruction) -> Result<u16, String> {
    compile_operation(instruction, BITWISE_SUB)
}

fn compile_operation(instruction: &Instruction, opcode: u16) -> Result<u16, String> {
    expect_arg_count(instruction, 3)?;
    let arg1 = instruction.args[0];
    let arg2 = instruction.args[1];
    let arg3 = instruction.args[2];

    match (arg1, arg2, arg3) {
        (Arg::Register(dest), Arg::Register(a), Arg::Register(b)) => {
            Ok(opcode | dest.u16() << DEST_POS | a.u16() << ARG_A_POS | b.u16() << ARG_B_POS)
        }
        _ => {
            return Err("Can only do this operation on registers".into());
        }
    }
}

pub fn expect_arg_count(instruction: &Instruction, amount: usize) -> Result<(), String> {
    if instruction.args.len() != amount {
        return Err(format!(
            "Instruction {} expects {} arguments. Received {}",
            instruction.name,
            amount,
            instruction.args.len()
        ));
    }
    Ok(())
}
