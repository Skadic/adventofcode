pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "1
10
100
2024";

pub const SAMPLE2: &str = "1
2
3
2024";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn next_secret_number(mut prev: usize) -> usize {
    const MASK: usize = (1 << 24) - 1;
    prev = ((prev << 6) ^ prev) & MASK;
    prev = ((prev >> 5) ^ prev) & MASK;
    prev = ((prev << 11) ^ prev) & MASK;

    // sign, exponent, mantisse
    // value = sign * mantisse * 2^exponent %
    // mod = mantisse * 2^exponent - floor(mantisse * 2^exponent / 2^n) * 2^n
    // mantisse * 2^exponent - floor(mantisse * (2^exponent-n)) * 2^n
    // float / 2^n -> exponent - n

    prev
}

pub fn nth_secret_number(number: usize, n: usize) -> usize {
    (0..n).fold(number, |acc, _| next_secret_number(acc))
}
