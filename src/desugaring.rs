use crate::{Arg, Expr, Instruction, Line, Register};

pub fn desugar_lines(lines: Vec<Line>) -> Option<Vec<Line>> {
    let mut result = vec![];

    for line in lines {
        match &line.expr {
            Expr::Instruction(instruction) => match instruction.name.as_str() {
                "jmp" | "je" | "jne" | "jg" | "jng" | "jl" | "jnl" => {
                    if desugar_jump(&mut result, &line, &instruction) {
                        continue;
                    }
                }

                "call" => {
                    desugar_function_call(&mut result, &line, instruction)?;
                    continue;
                }

                "ret" => {
                    desugar_return(&mut result, &line, instruction)?;
                    continue;
                }

                "push" => {
                    desugar_push(&mut result, &line, instruction)?;
                    continue;
                }

                "pop" => {
                    desugar_pop(&mut result, &line, instruction)?;
                    continue;
                }

                "mov" => {
                    if desugar_constant_move(&mut result, &line, instruction) {
                        continue;
                    }
                }

                "nand" | "or" | "and" | "nor" | "add" | "sub" | "cmp" => {
                    if desugar_constant_move(&mut result, &line, instruction) {
                        continue;
                    }
                }

                _ => {}
            },
            Expr::Tag(_) => {}
        }
        result.push(line);
    }

    Some(result)
}

fn desugar_jump(result: &mut Vec<Line>, line: &Line, instruction: &Instruction) -> bool {
    if instruction.args.len() != 1 || !matches!(instruction.args[0], Arg::Tag(_)) {
        return false;
    }

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::TX), instruction.args[0].clone()],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: instruction.name.clone(),
            args: vec![],
        }),
    });
    true
}

fn desugar_function_call(
    result: &mut Vec<Line>,
    line: &Line,
    instruction: &Instruction,
) -> Option<()> {
    if instruction.args.len() != 1 || !matches!(instruction.args[0], Arg::Tag(_)) {
        println!(
            "Compile error on line {}: 'Call instruction takes in one tag'",
            line.line_number
        );
        return None;
    }

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::TX), Arg::Constant(10)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "add".into(),
            args: vec![Arg::Register(Register::TX), Arg::Register(Register::RIP)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Ram(Register::RSP), Arg::Register(Register::TX)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::TX), Arg::Constant(2)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "add".into(),
            args: vec![Arg::Register(Register::RSP), Arg::Register(Register::TX)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::TX), instruction.args[0].clone()],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "jmp".into(),
            args: vec![],
        }),
    });

    Some(())
}

fn desugar_return(result: &mut Vec<Line>, line: &Line, instruction: &Instruction) -> Option<()> {
    if instruction.args.len() != 0 {
        println!(
            "Compile error on line {}: 'return takes in no arguments'",
            line.line_number
        );
        return None;
    }

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::TX), Arg::Constant(2)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "sub".into(),
            args: vec![Arg::Register(Register::RSP), Arg::Register(Register::TX)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::RIP), Arg::Ram(Register::RSP)],
        }),
    });

    Some(())
}

fn desugar_push(result: &mut Vec<Line>, line: &Line, instruction: &Instruction) -> Option<()> {
    if instruction.args.len() != 1 {
        println!(
            "Compile error on line {}: 'Push expects 1 argument'",
            line.line_number
        );
        return None;
    }

    let register = match instruction.args[0] {
        Arg::Constant(_) | Arg::Tag(_) | Arg::Ram(_) => {
            result.push(Line {
                line_number: line.line_number,
                expr: Expr::Instruction(Instruction {
                    name: "mov".into(),
                    args: vec![Arg::Register(Register::TX), instruction.args[0].clone()],
                }),
            });
            Register::TX
        }
        Arg::Register(r) => r,
    };

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Ram(Register::RSP), Arg::Register(register)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::TX), Arg::Constant(2)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "add".into(),
            args: vec![Arg::Register(Register::RSP), Arg::Register(Register::TX)],
        }),
    });

    Some(())
}

fn desugar_pop(result: &mut Vec<Line>, line: &Line, instruction: &Instruction) -> Option<()> {
    if instruction.args.len() != 1 {
        println!(
            "Compile error on line {}: 'Pop expects one argument'",
            line.line_number
        );
        return None;
    }

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::TX), Arg::Constant(2)],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "sub".into(),
            args: vec![Arg::Register(Register::RSP), Arg::Register(Register::TX)],
        }),
    });

    let register = match instruction.args[0] {
        Arg::Constant(_) | Arg::Tag(_) | Arg::Ram(_) => Register::TX,
        Arg::Register(r) => r,
    };

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(register), Arg::Ram(Register::RSP)],
        }),
    });

    match instruction.args[0] {
        Arg::Ram(_) => {
            result.push(Line {
                line_number: line.line_number,
                expr: Expr::Instruction(Instruction {
                    name: "mov".into(),
                    args: vec![instruction.args[0].clone(), Arg::Register(Register::TX)],
                }),
            });
        }
        _ => {}
    }

    Some(())
}

fn desugar_constant_move(result: &mut Vec<Line>, line: &Line, instruction: &Instruction) -> bool {
    if instruction.args.len() != 2 || !matches!(instruction.args[1], Arg::Constant(_)) {
        return false;
    }

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: "mov".into(),
            args: vec![Arg::Register(Register::TX), instruction.args[1].clone()],
        }),
    });

    result.push(Line {
        line_number: line.line_number,
        expr: Expr::Instruction(Instruction {
            name: instruction.name.clone(),
            args: vec![instruction.args[0].clone(), Arg::Register(Register::TX)],
        }),
    });

    true
}
