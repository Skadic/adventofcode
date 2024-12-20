use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

pub mod part1;
pub mod part2;

pub struct Input {
    map: Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Input {
    let mut map = input
        .lines()
        .map(str::chars)
        .map(Iterator::collect::<Vec<_>>)
        .collect::<Vec<_>>();

    let mut start = None;
    for (x, y) in (0..map[0].len()).cartesian_product(0..map.len()) {
        match map[y][x] {
            'S' => {
                map[y][x] = '.';
                start = Some((x, y));
            }
            'E' => {
                map[y][x] = '.';
            }
            _ => {}
        }
    }

    Input {
        map,
        start_x: start.unwrap().0,
        start_y: start.unwrap().1,
    }
}

fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
    [
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
}

fn bfs(chars: &[Vec<char>], x: usize, y: usize) -> HashMap<(usize, usize), usize> {
    let mut queue = VecDeque::new();
    queue.push_back((x, y, 0usize));
    let mut visited = HashMap::new();
    visited.insert((x, y), 0);

    while let Some((current_x, current_y, dist)) = queue.pop_front() {
        neighbors(current_x, current_y)
            .into_iter()
            .filter(|pos| !visited.contains_key(pos))
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|(x, y)| {
                chars
                    .get(y)
                    .and_then(|row| row.get(x))
                    .map(|&v| ((x, y), v))
            })
            .filter(|&(_, v)| v != '#')
            .map(|((x, y), _)| (x, y, dist + 1))
            .for_each(|v @ (x, y, d)| {
                queue.push_back(v);
                visited.insert((x, y), d);
            });
    }

    visited
}
