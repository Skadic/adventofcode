use day06::count;

fn main() {
    let input = std::fs::read_to_string("../../inputs/day06/input.txt").unwrap();
    let (times, distances) = parse_input(input.as_ref());

    println!("part2: {}", count(times, distances))
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (vec![time], vec![distance])
}
