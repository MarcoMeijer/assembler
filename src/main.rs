mod compile_8bit;
mod compiler;
mod desugaring;
mod instruction;
mod parsing;
mod tag_resolver;
mod virtual_machine;

pub use compile_8bit::*;
pub use compiler::*;
use desugaring::desugar_lines;
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

    /// if it should run the assembly inside a virtual machine
    #[argh(switch, short = 'r')]
    run: bool,
}

fn u16_to_u8_array(array: &[u16]) -> Vec<u8> {
    array.into_iter().flat_map(|x| x.to_be_bytes()).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: AssemblerArgs = argh::from_env();

    let content: String = fs::read_to_string("example.asm")?;

    let Some(lines) = parse_string(&content) else {
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

    let Some(mut lines) = desugar_lines(lines) else {
        return Err(std::io::Error::other("Compile error").into());
    };

    resolve_tags(&mut lines, 2);

    let Some(binary) = compile_lines(&lines) else {
        return Err(std::io::Error::other("Compile error").into());
    };

    if args.run {
        let binary = u16_to_u8_array(&binary);
        virtual_machine::program_loop(binary);
    } else {
        for instruction in &binary {
            println!("{instruction:016b}");
        }
    }

    Ok(())
}
