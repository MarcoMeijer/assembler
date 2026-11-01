use nom::{
    Err, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, terminated},
};

use crate::{Arg, Expr, Instruction, Line, Register, Tag};

fn remove_comment(line: &str) -> &str {
    if let Some(i) = line.find(';') {
        line.split_at(i).0
    } else {
        line
    }
}

type StrErr<'a> = nom::error::Error<&'a str>;

fn parse_literal<T: Clone + Copy>(
    word: &str,
    value: T,
) -> impl Parser<&str, Output = T, Error = StrErr> {
    map(tag::<&str, &str, StrErr>(word), move |_| value)
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    alt((
        parse_literal("reg1", Register::Reg1),
        parse_literal("reg2", Register::Reg2),
        parse_literal("reg3", Register::Reg3),
        parse_literal("reg4", Register::Reg4),
        parse_literal("reg5", Register::Reg5),
        parse_literal("reg6", Register::Reg6),
        parse_literal("rsp", Register::RSP),
    ))
    .parse(input)
}

fn parse_number(input: &str) -> IResult<&str, u16> {
    map(digit1, |a: &str| a.parse().unwrap()).parse(input)
}

fn parse_memory(input: &str) -> IResult<&str, Register> {
    delimited(tag("["), parse_register, tag("]")).parse(input)
}

fn parse_arg(input: &str) -> IResult<&str, Arg> {
    alt((
        map(parse_register, |x| Arg::Register(x)),
        map(parse_memory, |x| Arg::Ram(x)),
        map(parse_number, |x| Arg::Constant(x)),
    ))
    .parse(input)
}

fn parse_word(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn ws(input: &str) -> IResult<&str, &str> {
    multispace0.parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        pair(
            terminated(parse_word, ws),
            separated_list1(terminated(tag(","), ws), terminated(parse_arg, ws)),
        ),
        |(tag, args)| Instruction {
            name: tag.to_string(),
            args,
        },
    )
    .parse(input)
}

fn parse_tag(input: &str) -> IResult<&str, Tag> {
    map(terminated(parse_word, tag(":")), |tag| Tag(tag.into())).parse(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(parse_instruction, |i| Expr::Instruction(i)),
        map(parse_tag, |i| Expr::Tag(i)),
    ))
    .parse(input)
}

pub fn parse_string(content: &str) -> Option<Vec<Line>> {
    let x: Vec<_> = content
        .lines()
        .map(|x| remove_comment(x))
        .map(|x| x.trim())
        .enumerate()
        .map(|(x, y)| (x + 1, y))
        .filter(|(_, x)| !x.is_empty())
        .collect();

    let mut found_error = false;
    let mut lines = vec![];

    for (line, s) in x {
        let parse_result = parse_expr(s);

        if let Err(err) = parse_result {
            match err {
                Err::Incomplete(_) => {
                    println!("Parsing error on line {line}: input is incomplete");
                    found_error = true;
                }
                Err::Error(e) => {
                    let input = e.input;
                    println!("Parsing error on line {line}: Couldn't parse '{input}'");
                    found_error = true;
                }
                Err::Failure(_) => todo!(),
            }
            continue;
        }

        let Ok(result) = parse_result else {
            continue;
        };

        if !result.0.is_empty() {
            println!(
                "Parsing error on line {line}: Couldn't parse '{}'",
                result.0
            );
            found_error = true;
        }

        lines.push(Line {
            line_number: line,
            expr: result.1,
        });
    }

    if found_error { None } else { Some(lines) }
}
