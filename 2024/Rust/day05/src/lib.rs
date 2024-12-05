pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Orders {
    rules: Vec<(usize, usize)>,
    pages: Vec<Vec<usize>>,
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Orders {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(pred, succ)| (pred.parse().unwrap(), succ.parse().unwrap()))
        .collect();

    let pages = pages
        .lines()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();

    Orders { rules, pages }
}
