
use winnow::{
    ascii::{digit1, multispace0},
    combinator::{preceded, repeat, separated_pair, terminated},
    token::one_of,
    PResult, Parser,
};

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
pub const SAMPLE2: &str = "Button A: X+20, Y+29
Button B: X+80, Y+31
Prize: X=2020, Y=1399";

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Prize {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Prize> {
    repeat(1.., terminated(prize, multispace0))
        .parse(input)
        .unwrap()
}

fn isizee(input: &mut &str) -> PResult<isize> {
    digit1
        .try_map(|v: &str| v.parse::<isize>())
        .parse_next(input)
}

fn prize(input: &mut &str) -> PResult<Prize> {
    let mut button = preceded(
        ("Button ", one_of(['A', 'B']), ": "),
        separated_pair(preceded("X+", isizee), ", ", preceded("Y+", isizee)),
    );
    let mut prizee = preceded(
        "Prize: ",
        separated_pair(
            preceded("X=", isizee),
            ", ",
            preceded("Y=", isizee),
        ),
    );
    let a = button.parse_next(input)?;
    multispace0.parse_next(input)?;
    let b = button.parse_next(input)?;
    multispace0.parse_next(input)?;
    let prize = prizee.parse_next(input)?;

    Ok(Prize { a, b, prize })

    //winnow::combinator::seq! {Prize { a: button, _: line_ending, b: button, _: line_ending, prize: prizee }}.parse_next(input)
}
