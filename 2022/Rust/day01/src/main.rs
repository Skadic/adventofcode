fn main() {
    let input = include_str!("../res/input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap_or_default()
}

fn part2(input: &str) -> usize {
    let mut elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum::<usize>())
        .collect::<Vec<_>>();
    elves.sort_unstable();
    elves.into_iter().rev().take(3).sum()
}
