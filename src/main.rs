mod instruction;
mod parsing;

pub use instruction::*;
pub use parsing::*;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string("example.asm")?;

    parse_string(&content);

    Ok(())
}
