mod compile_8bit;
mod compiler;
mod instruction;
mod parsing;
mod tag_resolver;

pub use compile_8bit::*;
pub use compiler::*;
pub use instruction::*;
pub use parsing::*;
use tag_resolver::resolve_tags;

use std::error::Error;
use std::fs;

use argh::FromArgs;

#[derive(FromArgs)]
/// Simple assembly language compiler
struct AssemblerArgs {
    /// if using the 8bit instructions or not
    #[argh(switch, short = 'b')]
    bit8: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: AssemblerArgs = argh::from_env();

    let content: String = fs::read_to_string("example.asm")?;

    let Some(mut lines) = parse_string(&content) else {
        return Err(std::io::Error::other("Parse error").into());
    };

    if args.bit8 {
        let Some(binary) = compile_lines_8bit(&lines) else {
            return Err(std::io::Error::other("Compile error").into());
        };

        for instruction in binary {
            println!("{instruction:08b}");
        }
        return Ok(());
    }

    resolve_tags(&mut lines, 2);

    let Some(binary) = compile_lines(&lines) else {
        return Err(std::io::Error::other("Compile error").into());
    };

    for instruction in binary {
        println!("{instruction:016b}");
    }

    Ok(())
}
