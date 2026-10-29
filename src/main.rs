mod compiler;
mod instruction;
mod parsing;

pub use compiler::*;
pub use instruction::*;
pub use parsing::*;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string("example.asm")?;

    let Some(lines) = parse_string(&content) else {
        return Err(std::io::Error::other("Parse error").into());
    };

    let Some(binary) = compile_lines(&lines) else {
        return Err(std::io::Error::other("Compile error").into());
    };

    for instruction in binary {
        println!("{instruction:016b}");
    }

    Ok(())
}
