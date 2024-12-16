use std::{cmp::Reverse, collections::BinaryHeap};

use tracing::info;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

pub const SAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

pub const SAMPLE3: &str = "###########
#########E#
#########.#
#########.#
####......#
####.####.#
#S........#
###########
";

pub const SAMPLE4: &str = "######
####E#
#....#
#.#.##
#...##
#S####
######
";

pub const SAMPLE5: &str = "#####
###E#
###.#
#...#
#.#.#
#...#
#S###
#####
";

pub const SAMPLE6: &str = "##################################
#.....#.........#.......#.....##E#
#.#.#.#.#####.###.#.#####.###.##.#
#.#.#.#.....#...#.#.#.......#.#..#
#.#.#######.###.#.#.#.#######.#..#
#.#.#.....#...#...#.......#...#..#
###.#.#.#####.#############.####.#
#...#.#..........................#
#.#.#.###.#.#.#.#####.#.#####.##.#
#...................#.#.#...#.#..#
#.###.#.#.###.###.#.#.###.#.#.#..#
#.#...#.#.....#...#...#...#...#..#
#.#.#.#.###.###.#####.#.########.#
#...#.#.......#.....#.#.#.....#..#
#.###.#.#.#.#.#####.###.###.###..#
#.#.....#.#...#...#...#...#.#....#
#.#.#.###.#.###.#.###.###.#.#.#..#
#.#.......#.....#.#.....#.....#..#
#.#.#.#####.#######.###.###.#.#..#
#.#.#.......#.......#.#.#.#.#.#..#
#.#.#.#####.#.#######.#.#.#.#.##.#
#...#.#.....#.....#.......#.#....#
###.#.#.###.#####.#.#######.######
#.............#...#.#.......#....#
###.#.#.###.#.#.#.#.#.#######.#..#
#...#.....#.#.#.#...#.......#....#
#S#.#.###.#.#.#.###########.#.#..#
##################################";
pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(str::chars)
        .map(|chars| chars.map(|c| if c == 'E' || c == 'S' { '.' } else { c }))
        .map(Iterator::collect)
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_horiz(self) -> bool {
        matches!(self, Self::Down | Self::Up)
    }
    fn opposite(self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Dist {
    x: usize,
    y: usize,
    dist: DistType,
    last_move: Direction,
}

impl Dist {
    fn new(x: usize, y: usize, dist: DistType, moving_dir: Direction) -> Self {
        Self {
            x,
            y,
            dist,
            last_move: moving_dir,
        }
    }
}

impl PartialOrd for Dist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.dist.cmp(&other.dist))
    }
}

impl Ord for Dist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

pub type DistType = usize;

pub fn dijkstra1(map: &[Vec<char>], start_x: usize, start_y: usize) -> Vec<Vec<DistType>> {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Dist::new(start_x, start_y, 0, Direction::Right)));
    let mut dists = vec![vec![DistType::MAX; map[0].len()]; map.len()];
    let is_free = |x: usize, y: usize| map[y][x] == '.';
    dists[start_y][start_x] = 0;

    while let Some(Reverse(pos)) = queue.pop() {
        dists[pos.y][pos.x] = pos.dist;
        for (i, (x, y, current_move)) in [
            (pos.x - 1, pos.y, Direction::Left),
            (pos.x + 1, pos.y, Direction::Right),
            (pos.x, pos.y - 1, Direction::Up),
            (pos.x, pos.y + 1, Direction::Down),
        ]
        .into_iter()
        .enumerate()
        {
            if !is_free(x, y) || current_move == pos.last_move.opposite() {
                continue;
            }

            let extra_dist = if pos.last_move == current_move {
                1 as DistType
            } else {
                1001
            };
            if dists[y][x] > pos.dist + extra_dist {
                queue.push(Reverse(Dist::new(
                    x,
                    y,
                    pos.dist + extra_dist,
                    current_move,
                )));
            }
        }
    }

    dists
}

pub fn dijkstra(
    map: &[Vec<char>],
    start_x: usize,
    start_y: usize,
) -> Vec<Vec<(DistType, DistType)>> {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Dist::new(start_x, start_y, 0, Direction::Right)));
    let mut dists = vec![vec![(DistType::MAX, DistType::MAX); map[0].len()]; map.len()];
    let is_free = |x: usize, y: usize| map[y][x] == '.';
    dists[start_y][start_x] = (0, 0);

    while let Some(Reverse(pos)) = queue.pop() {
        for (i, (x, y, current_move)) in [
            (pos.x - 1, pos.y, Direction::Left),
            (pos.x + 1, pos.y, Direction::Right),
            (pos.x, pos.y - 1, Direction::Up),
            (pos.x, pos.y + 1, Direction::Down),
        ]
        .into_iter()
        .enumerate()
        {
            if !is_free(x, y) || current_move == pos.last_move.opposite() {
                continue;
            }

            let extra_dist = if pos.last_move == current_move {
                1 as DistType
            } else {
                1001
            };
            match current_move {
                Direction::Left | Direction::Right if dists[y][x].0 > pos.dist + extra_dist => {
                    dists[y][x].0 = pos.dist + extra_dist;
                    queue.push(Reverse(Dist::new(
                        x,
                        y,
                        pos.dist + extra_dist,
                        current_move,
                    )));
                }
                Direction::Up | Direction::Down if dists[y][x].1 > pos.dist + extra_dist => {
                    dists[y][x].1 = pos.dist + extra_dist;
                    queue.push(Reverse(Dist::new(
                        x,
                        y,
                        pos.dist + extra_dist,
                        current_move,
                    )));
                }
                _ => {}
            }
        }
    }

    dists
}
