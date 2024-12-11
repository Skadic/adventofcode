use std::collections::HashMap;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "125 17";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}

pub fn step(input: &mut HashMap<usize, usize>) {
    let mut output = HashMap::with_capacity(input.len() * 2);
    for (&stone, &old_count) in input.iter() {
        if stone == 0 {
            *output.entry(1).or_insert(0) += old_count;
        } else if num_digits(stone) % 2 == 0 {
            let (l, r) = split_digits(stone);
            *output.entry(l).or_insert(0) += old_count;
            *output.entry(r).or_insert(0) += old_count;
        } else {
            *output.entry(stone * 2024).or_insert(0) += old_count;
        }
    }

    std::mem::swap(input, &mut output);
}

pub fn num_digits(n: usize) -> usize {
    (n as f64 + 1.0).log10().ceil() as usize
}

pub fn split_digits(n: usize) -> (usize, usize) {
    let digits = num_digits(n) / 2;
    let factor = 10usize.pow(digits as u32);
    (n / factor, n % factor)
}
