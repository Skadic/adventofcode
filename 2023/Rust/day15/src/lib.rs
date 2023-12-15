pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) {
    let _ = input;
    todo!("parse input")
}

