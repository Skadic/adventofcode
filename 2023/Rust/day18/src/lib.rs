use std::fmt::Display;

use tracing::{info, info_span};

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rev<T>(T);

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T> From<T> for Rev<T> {
    fn from(value: T) -> Self {
        Rev(value)
    }
}

impl<T: Display> Display for Rev<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Direction {
    Empty,
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    pub fn is_vert(&self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<(Direction, usize, (Direction, usize))> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let dir = match split.next().unwrap() {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                s => panic!("invalid input: \"{s}\""),
            };
            let dist = split.next().unwrap().parse().unwrap();
            let rgb = {
                let s = split.next().unwrap();
                let s = &s[2..s.len() - 1];
                let dist = usize::from_str_radix(&s[..s.len() - 1], 16).unwrap();
                let dir = match usize::from_str_radix(&s[s.len() - 1..], 16).unwrap() {
                    0 => Direction::Right,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Up,
                    _ => panic!("invalid"),
                };
                (dir, dist)
            };

            (dir, dist, rgb)
        })
        .collect()
}

pub fn path(
    input: &[(Direction, usize, (Direction, usize))],
) -> (Vec<(usize, usize)>, Vec<Vec<Direction>>) {
    let mut v = vec![(0, 0)];
    let mut current = (0isize, 0isize);
    let mut max = (isize::MIN, isize::MIN);
    let mut min = (isize::MAX, isize::MAX);

    for &(dir, dist, _) in input {
        let dist = dist as isize;
        match dir {
            Direction::Left => current.0 -= dist,
            Direction::Right => current.0 += dist,
            Direction::Up => current.1 -= dist,
            Direction::Down => current.1 += dist,
            _ => panic!(),
        }
        v.push(current);
        max.0 = max.0.max(current.0);
        max.1 = max.1.max(current.1);
        min.0 = min.0.min(current.0);
        min.1 = min.1.min(current.1);
    }

    let (offset_x, offset_y) = min;

    let v = v
        .into_iter()
        .map(|(x, y)| ((x - offset_x) as usize, (y - offset_y) as usize))
        .collect::<Vec<_>>();
    let max = ((max.0 - offset_x) as usize, (max.1 - offset_y) as usize);

    let mut map = vec![vec![Direction::Empty; max.0 + 1]; max.1 + 1];
    for win in v.windows(2) {
        let (x1, y1) = win[0];
        let (x2, y2) = win[1];
        match 0 {
            _ if x1 < x2 => {
                for x in x1..=x2 {
                    if !map[y1][x].is_vert() {
                        map[y1][x] = Direction::Right
                    }
                }
            }
            _ if x1 > x2 => {
                for x in x2..=x1 {
                    if !map[y1][x].is_vert() {
                        map[y1][x] = Direction::Left
                    }
                }
            }
            _ if y1 < y2 => {
                for y in y1..=y2 {
                    if !map[y][x1].is_vert() {
                        map[y][x1] = Direction::Down
                    }
                }
            }
            _ if y1 > y2 => {
                for y in y2..=y1 {
                    if !map[y][x1].is_vert() {
                        map[y][x1] = Direction::Up
                    }
                }
            }
            _ => panic!(),
        };
    }

    (v, map)
}
pub fn path2(
    input: &[(Direction, usize, (Direction, usize))],
) -> (Vec<(usize, usize)>, (usize, usize)) {
    let mut v = vec![(0, 0)];
    let mut current = (0isize, 0isize);
    let mut max = (isize::MIN, isize::MIN);
    let mut min = (isize::MAX, isize::MAX);

    for &(_, _, (dir, dist)) in input {
        let dist = dist as isize;
        match dir {
            Direction::Left => current.0 -= dist,
            Direction::Right => current.0 += dist,
            Direction::Up => current.1 -= dist,
            Direction::Down => current.1 += dist,
            _ => panic!("aua"),
        }
        v.push(current);
        max.0 = max.0.max(current.0);
        max.1 = max.1.max(current.1);
        min.0 = min.0.min(current.0);
        min.1 = min.1.min(current.1);
    }

    let (offset_x, offset_y) = min;

    let v = v
        .into_iter()
        .map(|(x, y)| ((x - offset_x) as usize, (y - offset_y) as usize))
        .collect::<Vec<_>>();
    let bounds = (
        (max.0 - offset_x + 1) as usize,
        (max.1 - offset_y + 1) as usize,
    );

    (v, bounds)
}

pub fn count(map: &[Vec<Direction>]) -> usize {
    let mut cnt = 0;
    for line in map {
        let mut inside = false;
        let mut last_was_down = false;
        for &dir in line {
            if dir == Direction::Empty && last_was_down {
                inside = false;
            } else if dir == Direction::Up {
                inside = true;
            }
            cnt += if inside { 1 } else { 0 };
            if dir == Direction::Down {
                last_was_down = true;
            } else {
                last_was_down = false;
            }
        }
    }

    cnt
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let minx = self.x1.min(self.x2);
        let other_minx = other.x1.min(other.x2);
        let miny = self.y1.min(self.y2);
        let other_miny = other.y1.min(other.y2);

        minx.cmp(&other_minx).then_with(|| miny.cmp(&other_miny))
    }
}

impl Line {
    pub fn direction(self) -> Direction {
        let x1 = self.x1;
        let x2 = self.x2;
        let y1 = self.y1;
        let y2 = self.y2;
        match 0 {
            _ if x1 < x2 => Direction::Right,
            _ if x1 > x2 => Direction::Left,
            _ if y1 < y2 => Direction::Down,
            _ if y1 > y2 => Direction::Up,
            _ => panic!("could not get direction"),
        }
    }
}

pub fn count4(corner_points: &[(usize, usize)]) -> usize {
    let mut lines = corner_points
        .windows(2)
        .map(|win| {
            let (x1, y1) = win[0];
            let (x2, y2) = win[1];
            Line { x1, y1, x2, y2 }
        })
        .filter(|&l| l.direction() != Direction::Right && l.direction() != Direction::Left)
        .collect::<Vec<_>>();
    lines.sort_by_key(|l| l.x1.min(l.x2));

    let mut sum = 0usize;

    let mut active_ranges = vec![];
    let mut last_x = 0;
    for line in lines {
        let x = line.x1;
        let min_y = line.y1.min(line.y2);
        let max_y = line.y1.max(line.y2);

        let range_cover = active_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum::<usize>();
        let inc = (x - last_x) * range_cover;
        sum += inc;

        match line.direction() {
            Direction::Up => merge_into(&mut active_ranges, (min_y, max_y)),
            Direction::Down => {
                let removed_edges = remove_from(&mut active_ranges, (min_y, max_y));
                sum += max_y - min_y - 1 + removed_edges;
            }
            _ => {}
        }

        for win in active_ranges.windows(2) {
            let (a1, b1) = win[0];
            let (a2, b2) = win[1];
            assert!(a1 <= b1);
            assert!(b1 < a2);
            assert!(a2 <= b2);
        }

        last_x = x;
    }

    sum
}

pub fn merge_into(v: &mut Vec<(usize, usize)>, (s, e): (usize, usize)) {
    let _span = info_span!("merge").entered();
    if v.is_empty() {
        v.push((s, e));
        return;
    }

    if v[0].0 == e {
        v[0].0 = s;
        return;
    }

    if e < v[0].0 {
        v.insert(0, (s, e));
        return;
    }

    if s == v.last().unwrap().1 {
        v.last_mut().unwrap().1 = e;
        return;
    }

    if s > v.last().unwrap().1 {
        v.push((s, e));
        return;
    }

    let (insert_pos, _) = v.iter().enumerate().rev().find(|(_, l)| l.0 <= s).unwrap();

    if s > v.last().unwrap().1 {
        v.push((s, e));
        return;
    }

    if v[insert_pos].0 <= s && e <= v[insert_pos].1 {
        return;
    }

    let mut merged_front = false;
    if s == v[insert_pos].1 {
        v[insert_pos].1 = e;
        merged_front = true;
    }

    let mut merged_back = false;
    if insert_pos < v.len() - 1 && e == v[insert_pos + 1].0 {
        if merged_front {
            v[insert_pos].1 = v[insert_pos + 1].1;
            v.remove(insert_pos + 1);
        } else {
            v[insert_pos + 1].0 = s;
        }
        merged_back = true;
    }

    if !merged_front && !merged_back {
        v.insert(insert_pos + 1, (s, e));
    }
}

pub fn remove_from(v: &mut Vec<(usize, usize)>, (s, e): (usize, usize)) -> usize {
    let _span = info_span!("remove").entered();
    if v.is_empty() {
        return 0;
    }

    if let Some(eq_pos) = v.iter().position(|&l| l == (s, e)) {
        v.remove(eq_pos);
        return 2;
    }

    if let Some(prefix_pos) = v.iter().position(|l| l.0 == s) {
        v[prefix_pos].0 = e;
        return 1;
    }

    if let Some(suffix_pos) = v.iter().position(|l| l.1 == e) {
        v[suffix_pos].1 = s;
        return 1;
    }

    let pos = v.iter().position(|&l| l.0 < s && e < l.1).unwrap();

    let prev_start = v[pos].0;
    v[pos].0 = e;
    v.insert(pos, (prev_start, s));
    return 0;
}

pub fn print_map(map: &[Vec<Direction>]) {
    for (i, line) in map.iter().enumerate() {
        let s = line
            .iter()
            .map(|&dir| match dir {
                Direction::Left => '<',
                Direction::Right => '>',
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Empty => '.',
            })
            .collect::<String>();
        info!(l=%format!("{i:-3}"),map = %s);
    }
}
//.............................^<<<<<v...........^>>>>>^...........v>>>>>>>>v...............^.........v......^....v.......^...........................................................................................................................................................v..........^....v.......^...........................................v...........................................

//58
//.....................................................................................................................................................................................^........v............................................^.................v.............^...............v...^.....v......^<<<<<<<v...............................................................................
