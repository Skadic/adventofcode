pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(str::split_whitespace)
        .map(|split| split.map(|v| v.parse().unwrap()).collect())
        .collect()
}
