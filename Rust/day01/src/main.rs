fn main() {
    let values = std::fs::read_to_string("res/input.txt")
    .unwrap()
    .lines()
    .map(|s| s.trim().parse::<usize>().unwrap())
    .collect::<Vec<_>>();

    println!("part 1: {}", part1(&values));
    println!("part 2: {}", part2(&values));
}

fn part1(values: &[usize]) -> usize {
    values
        .iter()
        .zip(values.iter().skip(1))
        .filter(|(prev, next)| next > prev)
        .count()
}

fn part2(values: &[usize]) -> usize {
    let window_sums = values
        .iter()
        .zip(values.iter().skip(1))
        .zip(values.iter().skip(2))
        .map(|((v1, v2), v3)| v1 + v2 + v3)
        .collect::<Vec<_>>();

    window_sums
        .iter()
        .zip(window_sums.iter().skip(1))
        .filter(|(prev, next)| next > prev)
        .count()
}