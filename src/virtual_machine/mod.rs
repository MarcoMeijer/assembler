mod instruction;
mod ram;
mod register;

use std::cmp::Ordering;

pub use instruction::*;
pub use ram::*;
pub use register::*;

pub const INSTRUCTION_SIZE: usize = 2;

fn execute_instruction(registers: &Registers, ram: &mut Ram, instruction: Instruction) -> u16 {
    let a = read_register(registers, instruction.argument_a);
    let b = read_register(registers, instruction.argument_b);

    match instruction.mode {
        Mode::Constant(x) => x,
        Mode::ALU(opcode) => match opcode {
            Opcode::Nand => !(a & b),
            Opcode::Or => a | b,
            Opcode::And => a & b,
            Opcode::Nor => !(a | b),
            Opcode::Add => a.wrapping_add(b),
            Opcode::Sub => a.wrapping_sub(b),
            Opcode::Cmp => {
                let cmp = a.cmp(&b);
                match cmp {
                    Ordering::Less => 0b001,
                    Ordering::Equal => 0b010,
                    Ordering::Greater => 0b100,
                }
            }
            Opcode::Noop => 0,
        },
        Mode::IO(io) => match io {
            IO::ReadRamU8 => ram.get_u8(a) as u16,
            IO::ReadRamU16 => ram.get_u16(a),
            IO::WriteRamU8 => {
                ram.set_u8(a, b as u8);
                0
            }
            IO::WriteRamU16 => {
                ram.set_u16(a, b);
                0
            }
            IO::Output => {
                println!("{a}");
                0
            }
            IO::Noop => 0,
        },
        Mode::Jump(jmp) => {
            let cmp = registers.cmp_register;
            let jump = match jmp {
                Jump::Je => (cmp & 0b010) != 0,
                Jump::Jne => (cmp & 0b010) == 0,
                Jump::Jg => (cmp & 0b100) != 0,
                Jump::Jng => (cmp & 0b100) == 0,
                Jump::Jl => (cmp & 0b001) != 0,
                Jump::Jnl => (cmp & 0b001) == 0,
                Jump::Jmp => true,
                Jump::Noop => false,
            };
            if jump { a } else { registers.program_counter }
        }
        Mode::Noop => 0,
    }
}

pub fn program_loop(code: Vec<u8>) {
    let mut registers = Registers::default();
    let mut ram = Ram::default();

    loop {
        // 1. Fetch
        let pc = registers.program_counter as usize;
        if pc + INSTRUCTION_SIZE > code.len() {
            break;
        }
        registers.program_counter += INSTRUCTION_SIZE as u16;
        let instruction: u16 =
            u16::from_be_bytes(code[pc..pc + INSTRUCTION_SIZE].try_into().unwrap());

        // 2. Decode
        let instruction = decode_instruction(instruction);

        // 3. Execute
        let result = execute_instruction(&registers, &mut ram, instruction);

        // 4. Store
        store_register(&mut registers, instruction.destination, result);
    }
}
