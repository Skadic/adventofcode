pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Equation {
    target: usize,
    operands: Vec<usize>,
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let target = l.parse().unwrap();
            let operands = r.split_whitespace().map(|v| v.parse().unwrap()).collect();
            Equation { target, operands }
        })
        .collect()
}
