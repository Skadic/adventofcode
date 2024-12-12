use fxhash::FxHashMap as HashMap;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";
pub const SAMPLE2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

pub const SAMPLE3: &str = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

pub mod part1;
pub mod part2;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Plot {
    pub area: usize,
    pub perim: usize,
    pub sides: isize,
}
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Regions {
    pub ids: HashMap<char, usize>,
    pub plots: HashMap<(char, usize), Plot>,
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|v| !v.is_empty())
        .map(str::chars)
        .map(Iterator::collect)
        .collect()
}
