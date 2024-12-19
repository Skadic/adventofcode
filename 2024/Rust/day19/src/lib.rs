pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    input
        .split_once("\n\n")
        .map(|(towels, combos)| {
            (
                towels.split(",").map(str::trim).collect(),
                combos.lines().collect(),
            )
        })
        .unwrap()
}
