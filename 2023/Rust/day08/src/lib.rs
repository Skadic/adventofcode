use std::collections::HashMap;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

pub const SAMPLE2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> (Vec<usize>, Vec<&str>, HashMap<&str, [&str; 2]>) {
    let mut lines = input.lines();
    let rl = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect();

    lines.next();

    let (starts, dirs) = lines
        .map(|s| {
            let (src, dirs) = s.split_once('=').unwrap();
            let (l, r) = dirs.split_once(',').unwrap();
            let (l, r) = (l.trim(), r.trim());
            let (l, r) = (&l[1..], &r[..r.len() - 1]);
            (src.trim(), (src.trim(), [l.trim(), r.trim()]))
        })
        .unzip::<_, _, Vec<_>, HashMap<_, _>>();

    (
        rl,
        starts.into_iter().filter(|s| s.ends_with('A')).collect(),
        dirs,
    )
}
