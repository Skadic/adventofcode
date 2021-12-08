use std::{
    collections::{HashMap, HashSet, BTreeSet}
};

fn main() {
    let segments: [HashSet<char>; 10] = [
        ['a', 'b', 'c', 'e', 'f', 'g'].into_iter().collect(),
        ['c', 'f'].into_iter().collect(),
        ['a', 'c', 'd', 'e', 'g'].into_iter().collect(),
        ['a', 'c', 'd', 'f', 'g'].into_iter().collect(),
        ['b', 'c', 'd', 'f'].into_iter().collect(),
        ['a', 'b', 'd', 'f', 'g'].into_iter().collect(),
        ['a', 'b', 'd', 'e', 'f', 'g'].into_iter().collect(),
        ['a', 'c', 'f'].into_iter().collect(),
        ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect(),
        ['a', 'b', 'c', 'd', 'f', 'g'].into_iter().collect(),
    ];

    let segment_counts: [Vec<usize>; 8] = [
        vec![],
        vec![],
        vec![1],
        vec![7],
        vec![4],
        vec![2, 3, 5],
        vec![0, 6, 9],
        vec![8],
    ];

    let input = include_str!("../res/example.txt");

    let mut signal_patterns = input
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            split.next().unwrap()
        })
        .map(|sig| sig.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for pattern in signal_patterns.iter_mut() {
        pattern.sort_unstable_by(|&a, &b| a.len().cmp(&b.len()));
    }

    let output_patterns = input
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            split.nth(1).unwrap()
        })
        .map(|sig| sig.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        part1(
            &segments,
            &segment_counts,
            &signal_patterns,
            &output_patterns
        )
    );
}

fn part1(
    segments: &[HashSet<char>],
    segment_counts: &[Vec<usize>],
    signal_patterns: &[Vec<&str>],
    output_patterns: &[Vec<&str>],
) -> usize {

    
    for pattern_vec in signal_patterns {

        
        let mut possible_remaining = HashMap::new();

        for &pattern in pattern_vec {
            let len = pattern.len();
            let possible_digits = &segment_counts[len];
            let possible_segments = possible_digits.iter()
                .flat_map(|&digit| segments[digit].iter().cloned())
                .collect::<BTreeSet<_>>();
            
            for c in pattern.chars() {
                let clone = possible_segments.clone();
                let p_len = possible_remaining.len();
                possible_remaining.entry(c).or_insert(clone.clone());
                possible_remaining.entry(c).and_modify(|set| {
                    if p_len < set.len() {
                        *set = clone;
                    }
                });
            }
        }

        let mut possible_remaining = Vec::from_iter(possible_remaining.into_iter());

        possible_remaining.sort_unstable_by(|a, b| a.1.len().cmp(&b.1.len()));

        for (c, set) in possible_remaining.iter() {
            println!("{}: {:?}", c, set);
        }

        for i in 0..possible_remaining.len() {
            let (_, current_set) = possible_remaining[i].clone();
            for j in i + 1..possible_remaining.len() {
                let (_, cmp_set) = possible_remaining[j].clone();
                if cmp_set.len() > current_set.len() && current_set.iter().all(|c| cmp_set.contains(c)) {
                    possible_remaining[j].1.retain(|c| !current_set.contains(c));
                }
                println!("soos");
            }
        }
        for (c, set) in possible_remaining.iter() {
            println!("{}: {:?}", c, set);
        }
        
    }

    0
}