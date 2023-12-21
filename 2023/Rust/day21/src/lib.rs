use std::collections::{HashSet, VecDeque};

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "
...........
......##...
.#.#..#..#.
..#.#...#..
...........
.....S.....
.##......#.
........#..
.#..#.##.#.
..#...#.#..
...........";

pub mod part1;
pub mod part2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Garden,
    Rock,
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> (Vec<Vec<Tile>>, usize, usize) {
    let mut x = 0;
    let mut y = 0;
    let res = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Tile::Garden,
                    '#' => Tile::Rock,
                    'S' => {
                        x = col;
                        y = row;
                        Tile::Garden
                    }
                    _ => panic!("invalid char"),
                })
                .collect()
        })
        .collect::<Vec<_>>();
    (res, x, y)
}

pub fn bfs_part1(map: &[Vec<Tile>], x: usize, y: usize, target: usize) -> Vec<Vec<bool>> {
    let mut queue = VecDeque::new();
    let mut active = HashSet::new();

    queue.push_back((x, y));
    active.insert((x, y));

    let mut neighbors = [(0, 0); 4];
    for _ in 0..target {
        let mut new_active = HashSet::new();
        for (x, y) in std::mem::take(&mut queue) {
            neighbors[0] = if x > 0 && map[y][x - 1] == Tile::Garden {
                (x - 1, y)
            } else {
                (usize::MAX, usize::MAX)
            };
            neighbors[1] = if x < map[0].len() - 1 && map[y][x + 1] == Tile::Garden {
                (x + 1, y)
            } else {
                (usize::MAX, usize::MAX)
            };
            neighbors[2] = if y > 0 && map[y - 1][x] == Tile::Garden {
                (x, y - 1)
            } else {
                (usize::MAX, usize::MAX)
            };
            neighbors[3] = if y < map.len() - 1 && map[y + 1][x] == Tile::Garden {
                (x, y + 1)
            } else {
                (usize::MAX, usize::MAX)
            };

            for n in neighbors
                .into_iter()
                .filter(|&v| v != (usize::MAX, usize::MAX))
            {
                if !new_active.contains(&n) {
                    queue.push_back(n);
                    new_active.insert(n);
                }
            }
        }
        active = new_active;
    }
    let mut m = vec![vec![false; map[0].len()]; map.len()];

    for (x, y) in active {
        m[y][x] = true;
    }
    m
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reachable {
    Odd,
    Even,
    Never,
}

pub fn bfs(map: &[Vec<Tile>], x: usize, y: usize) -> Vec<Vec<Reachable>> {
    let mut queue = VecDeque::new();
    let mut done = HashSet::new();
    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];

    queue.push_back((x, y));
    done.insert((x, y));
    dist[y][x] = 0;

    let mut neighbors = [(0, 0); 4];
    while let Some((x, y)) = queue.pop_front() {
        let cur_dist = dist[y][x];
        neighbors[0] = if x > 0 && map[y][x - 1] == Tile::Garden {
            (x - 1, y)
        } else {
            (usize::MAX, usize::MAX)
        };
        neighbors[1] = if x < map[0].len() - 1 && map[y][x + 1] == Tile::Garden {
            (x + 1, y)
        } else {
            (usize::MAX, usize::MAX)
        };
        neighbors[2] = if y > 0 && map[y - 1][x] == Tile::Garden {
            (x, y - 1)
        } else {
            (usize::MAX, usize::MAX)
        };
        neighbors[3] = if y < map.len() - 1 && map[y + 1][x] == Tile::Garden {
            (x, y + 1)
        } else {
            (usize::MAX, usize::MAX)
        };

        for n @ (nx, ny) in neighbors
            .into_iter()
            .filter(|&v| v != (usize::MAX, usize::MAX))
        {
            if !done.contains(&n) {
                queue.push_back(n);
                done.insert((nx, ny));
                dist[ny][nx] = cur_dist + 1;
            }
        }
    }
    let mut m = vec![vec![Reachable::Never; map[0].len()]; map.len()];
    for (x, y) in done {
        m[y][x] = if dist[y][x] % 2 == 1 {
            Reachable::Odd
        } else {
            Reachable::Even
        };
    }
    m
}
