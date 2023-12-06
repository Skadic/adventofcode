use day06::count;

fn main() {
    let input = std::fs::read_to_string("../../inputs/day06/input.txt").unwrap();
    let (times, distances) = parse_input(input.as_ref());

    println!("part1: {}", count(times, distances))
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (times, distances)
}
