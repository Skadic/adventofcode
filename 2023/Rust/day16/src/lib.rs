use std::collections::HashSet;

use tracing::info;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Action {
    #[default]
    Empty,
    Init(Direction),
    Mirror(Direction),
    /// Order is left right for horizontal and up down for vertical
    Split(Axis),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn axis(self) -> Axis {
        match self {
            Self::Left | Self::Right => Axis::Horizontal,
            Self::Up | Self::Down => Axis::Vertical,
        }
    }

    /// Mirroring at '\'
    pub fn mirror_d(self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }

    /// Mirroring at '/'
    pub fn mirror_u(self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn dirs(self) -> [Direction; 2] {
        match self {
            Self::Horizontal => [Direction::Left, Direction::Right],
            Self::Vertical => [Direction::Up, Direction::Down],
        }
    }

    pub fn flip(self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Tile {
    #[default]
    Empty,
    MirrorU,
    MirrorD,
    Split(Axis),
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    use Tile::*;
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '/' => MirrorU,
                    '\\' => MirrorD,
                    '-' => Split(Axis::Horizontal),
                    '|' => Split(Axis::Vertical),
                    _ => panic!("invalid char: {c}"),
                })
                .collect()
        })
        .collect()
}

pub fn eval(
    map: &[Vec<Tile>],
    actions: impl IntoIterator<Item = (usize, usize, Action)>,
    w: usize,
    h: usize,
) -> usize {
    use Direction::*;
    let next_action = |x: usize, y: usize, dir: Direction| -> Option<(usize, usize, Direction)> {
        match dir {
            Left if x > 0 => Some((x - 1, y, Left)),
            Right if x + 1 < w => Some((x + 1, y, Right)),
            Up if y > 0 => Some((x, y - 1, Up)),
            Down if y + 1 < h => Some((x, y + 1, Down)),
            _ => None,
        }
    };
    let mut dirs = HashSet::new();
    for (x, y, act) in actions.into_iter() {
        match act {
            Action::Init(dir) => {
                dirs.insert((x, y, dir));
            }
            Action::Mirror(dir) => {
                if let Some(next) = next_action(x, y, dir) {
                    dirs.insert(next);
                }
            }
            Action::Split(axis) => axis
                .dirs()
                .into_iter()
                .filter_map(|v| next_action(x, y, v))
                .for_each(|a| {
                    dirs.insert(a);
                }),
            Action::Empty => {}
        }
    }

    let mut energized = vec![vec![false; w]; h];

    for (mut x, mut y, dir) in dirs.into_iter() {
        energized[y][x] = true;
        while map[y][x] == Tile::Empty || (map[y][x] == Tile::Split(dir.axis())) {
            energized[y][x] = true;
            match dir {
                Left if x > 0 => x -= 1,
                Right if x < w - 1 => x += 1,
                Up if y > 0 => y -= 1,
                Down if y < h - 1 => y += 1,
                _ => break,
            }
        }
        if let Some(r) = energized.get_mut(y).and_then(|line| line.get_mut(x)) {
            *r = true
        };
    }

    energized.into_iter().flat_map(|v| v).filter(|&v| v).count()
}

pub fn track(map: &[Vec<Tile>], start_x: usize, start_y: usize, start_dir: Direction) -> usize {
    use Action as A;
    use Direction::*;
    use Tile as T;

    let w = map[0].len();
    let h = map.len();

    let mut actions = HashSet::<(usize, usize, Action)>::new();
    actions.insert((start_x, start_y, A::Init(start_dir)));

    let mut action_queue = Vec::new();
    action_queue.push((start_x, start_y, start_dir));

    let next_action = |x: usize, y: usize, dir: Direction| -> Option<(usize, usize, Direction)> {
        match dir {
            Left if x > 0 => Some((x - 1, y, Left)),
            Right if x + 1 < w => Some((x + 1, y, Right)),
            Up if y > 0 => Some((x, y - 1, Up)),
            Down if y + 1 < h => Some((x, y + 1, Down)),
            _ => None,
        }
    };

    while !action_queue.is_empty() {
        let  (mut x, mut y, dir) = action_queue.pop().unwrap();
        let axis = dir.axis();

        while map[y][x] == T::Empty || (map[y][x] == T::Split(dir.axis())) {
            match dir {
                Left if x > 0 => x -= 1,
                Right if x < w - 1 => x += 1,
                Up if y > 0 => y -= 1,
                Down if y < h - 1 => y += 1,
                _ => break,
            }
        }

        match map[y][x] {
            T::MirrorD if !actions.contains(&(x, y, A::Mirror(dir.mirror_d()))) => {
                actions.insert((x, y, A::Mirror(dir.mirror_d())));
                if let Some(next) = next_action(x, y, dir.mirror_d()) {
                    action_queue.push(next)
                }
            }
            T::MirrorU if !actions.contains(&(x, y, A::Mirror(dir.mirror_u()))) => {
                actions.insert((x, y, A::Mirror(dir.mirror_u())));
                if let Some(next) = next_action(x, y, dir.mirror_u()) {
                    action_queue.push(next)
                }
            }
            T::Split(split_axis)
                if axis != split_axis && !actions.contains(&(x, y, A::Split(split_axis))) =>
            {
                actions.insert((x, y, A::Split(split_axis)));
                split_axis
                    .dirs()
                    .into_iter()
                    .filter_map(|d| next_action(x, y, d))
                    .for_each(|next| action_queue.push(next));
            }
            _ => continue,
        }
    }

    eval(&map, actions, w, h)
}

pub fn print_map(v: &[Vec<Tile>]) {
    for line in v {
        info!(map = %line.iter().map(|&t| match t {
            Tile::Empty => '.',
            Tile::MirrorD => '\\',
            Tile::MirrorU => '/',
            Tile::Split(Axis::Horizontal) => '-',
            Tile::Split(Axis::Vertical) => '|',
        }).collect::<String>())
    }
}
