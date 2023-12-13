pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut maps = vec![];
    let mut map = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            maps.push(std::mem::take(&mut map));
        } else {
            map.push(line.chars().collect())
        }
    }
    if !map.is_empty() {
        maps.push(map)
    }
    maps
}
