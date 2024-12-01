use std::iter;

const INPUT: &str = include_str!("../input.txt");
#[allow(unused)]
const SAMPLE: &str = include_str!("../sample.txt");

fn main() {
    let part1 = part1(INPUT);
    let part2 = part2(INPUT);

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut iter = line.split_whitespace().map(|nr| nr.parse().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
}

fn part1(input: &str) -> usize {
    let (mut left, mut right): (Vec<_>, Vec<_>) = parse_input(input).unzip();

    left.sort_unstable();
    right.sort_unstable();

    iter::zip(left, right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part2(input: &str) -> usize {
    let (left, right): (Vec<_>, Vec<_>) = parse_input(input).unzip();

    let max = *right.iter().max().unwrap_or(&0);

    let mut cnt = vec![0usize; max + 1];
    right.into_iter().for_each(|i| cnt[i] += 1);
    left.into_iter().map(|v| v * cnt[v]).sum()
}
