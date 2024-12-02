const INPUT: &str = include_str!("../input.txt");
const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const SAMPLE2: &str = "0 1 2 3 4 0";

fn main() {
    let input = parse_input(INPUT);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(input.clone()));
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|tok| tok.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(reports: &[Vec<usize>]) -> usize {
    reports
        .iter()
        .map(|levels| {
            levels
                .windows(2)
                .fold((isize::MAX, isize::MIN), |(min, max), v| {
                    let diff = (v[0] as isize) - (v[1] as isize);
                    (min.min(diff), max.max(diff))
                })
        })
        .filter(|&(min_diff, max_diff)| {
            -3 <= min_diff && max_diff <= -1 || 1 <= min_diff && max_diff <= 3
        })
        .count()
}

fn part2(reports: Vec<Vec<usize>>) -> usize {
    reports
        .into_iter()
        .filter(|levels| {
            (0..levels.len())
                .map(|i| {
                    let mut v = levels.to_vec();
                    v.remove(i);
                    v
                })
                .any(|levels| {
                    let (min_diff, max_diff) =
                        levels
                            .windows(2)
                            .fold((isize::MAX, isize::MIN), |(min, max), v| {
                                let diff = (v[0] as isize) - (v[1] as isize);
                                (min.min(diff), max.max(diff))
                            });
                    -3 <= min_diff && max_diff <= -1 || 1 <= min_diff && max_diff <= 3
                })
        })
        .count()
}

fn valid_asc(l: usize, r: usize) -> bool {
    l < r && (1..=3).contains(&(r - l))
}

fn valid_desc(l: usize, r: usize) -> bool {
    l > r && (1..=3).contains(&(l - r))
}
