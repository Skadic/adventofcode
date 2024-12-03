use winnow::{
    combinator::{alt, preceded, repeat, seq},
    prelude::*,
    token::{any, literal, take_while},
    Parser,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    let v: Vec<_> = repeat(
        0..,
        alt((
            alt((do_instr, dont_instr, mul_instr)).map(Some),
            any.map(|_| None),
        )),
    )
    .parse(input)
    .unwrap();
    v.into_iter().flatten().collect()
}

fn do_instr(input: &mut &str) -> PResult<Instruction> {
    literal("do()").map(|_| Instruction::Do).parse_next(input)
}

fn dont_instr(input: &mut &str) -> PResult<Instruction> {
    literal("don't()")
        .map(|_| Instruction::Dont)
        .parse_next(input)
}

fn mul_instr(input: &mut &str) -> PResult<Instruction> {
    preceded(
        literal("mul"),
        seq!(Instruction::Mul(_: '(', num, _: ',', num, _: ')')),
    )
    .parse_next(input)
}

fn num(input: &mut &str) -> PResult<usize> {
    take_while(1..=3, '0'..='9').parse_to().parse_next(input)
}
