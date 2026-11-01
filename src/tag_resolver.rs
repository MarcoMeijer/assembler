use std::collections::HashMap;

use crate::{Arg, Expr, Line};

pub fn resolve_tags(lines: &mut [Line], instruction_width: u16) {
    let mut tags = HashMap::new();
    let mut instruction_number = 0;

    for line in lines.iter() {
        match &line.expr {
            Expr::Instruction(_) => {
                instruction_number += instruction_width;
            }
            Expr::Tag(tag) => {
                tags.insert(tag.0.clone(), instruction_number);
            }
        }
    }

    for line in lines {
        let Expr::Instruction(instruction) = &mut line.expr else {
            continue;
        };

        for arg in &mut instruction.args {
            let Arg::Tag(tag) = arg else {
                continue;
            };

            let Some(instruction_number) = tags.get(tag) else {
                continue;
            };

            *arg = Arg::Constant(*instruction_number);
        }
    }
}
