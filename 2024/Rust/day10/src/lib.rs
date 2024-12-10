use std::{collections::{HashMap, HashSet, VecDeque}, ops::Add};

use tracing::info;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn dfs_trails(input: &[Vec<usize>]) -> usize {
    let zeroes = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter_map(move |(x, v)| if v == 0 { Some((x, y)) } else { None })
        })
        .collect::<VecDeque<_>>();
    let mut result = 0;

    for (z_x, z_y) in zeroes {
        let mut queue = VecDeque::new();
        queue.push_back((z_x, z_y));
        let mut visited_9s = HashSet::new();

        while let Some((current_x, current_y)) = queue.pop_back() {
            let val = input[current_y][current_x];
            if val == 9 {
                visited_9s.insert((current_x, current_y));
            } else {
                queue.extend(
                    [
                        (current_x - 1, current_y),
                        (current_x + 1, current_y),
                        (current_x, current_y - 1),
                        (current_x, current_y + 1),
                    ]
                    .into_iter()
                    .filter_map(|(x, y)| {
                        input
                            .get(y)
                            .and_then(|row| row.get(x))
                            .map(|&v| ((x, y), v))
                    })
                    .filter(|&(_, v)| v == val + 1)
                    .map(|(pos, _)| pos),
                );
            }
        }
        result += visited_9s.len();
    }
    result
}

pub fn dfs_trails2(input: &[Vec<usize>]) -> usize {
    let zeroes = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter_map(move |(x, v)| if v == 0 { Some((x, y)) } else { None })
        })
        .collect::<VecDeque<_>>();
    let mut result = 0;

    for (z_x, z_y) in zeroes {
        let mut queue = VecDeque::new();
        queue.push_back((z_x, z_y));
        let mut visited_9s = HashMap::new();

        while let Some((current_x, current_y)) = queue.pop_back() {
            let val = input[current_y][current_x];
            if val == 9 {
                *visited_9s.entry((current_x, current_y)).or_insert(0) += 1;
            } else {
                queue.extend(
                    [
                        (current_x - 1, current_y),
                        (current_x + 1, current_y),
                        (current_x, current_y - 1),
                        (current_x, current_y + 1),
                    ]
                    .into_iter()
                    .filter_map(|(x, y)| {
                        input
                            .get(y)
                            .and_then(|row| row.get(x))
                            .map(|&v| ((x, y), v))
                    })
                    .filter(|&(_, v)| v == val + 1)
                    .map(|(pos, _)| pos),
                );
            }
        }
        result += visited_9s.into_values().sum::<usize>();
    }
    result
}
