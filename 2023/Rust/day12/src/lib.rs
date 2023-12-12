pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

pub mod part1;
pub mod part2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<(Vec<SpringState>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (states, ranges) = line.split_once(' ').unwrap();
            (
                states
                    .chars()
                    .map(|c| match c {
                        '#' => SpringState::Damaged,
                        '.' => SpringState::Operational,
                        '?' => SpringState::Unknown,
                        _ => panic!("invalid spring state: '{c}'"),
                    })
                    .collect(),
                ranges.split(',').map(|r| r.parse().unwrap()).collect(),
            )
        })
        .collect()
}
