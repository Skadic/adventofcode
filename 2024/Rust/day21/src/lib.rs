#![allow(unused)]

use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use serde_json::json;
use tracing::info;

// 184712 TOO HIGH

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "029A
980A
179A
456A
379A";

pub mod part1;
pub mod part2;

const NUMPAD_POSSIBLE_MOVES: [&[char]; 11] = [
    &['^', '>'],
    &['^', '>'],
    &['^', '>', 'v', '<'],
    &['^', 'v', '<'],
    &['^', '>', 'v'],
    &['^', '>', 'v', '<'],
    &['^', 'v', '<'],
    &['>', 'v'],
    &['>', 'v', '<'],
    &['<', 'v'],
    &['^', '<'],
];

pub fn actions_possible_moves() -> HashMap<String, HashMap<String, Vec<String>>> {
    // ty https://www.reddit.com/user/Zefick/
    // https://www.reddit.com/r/adventofcode/comments/1hja685/comment/m35rvek/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    let value = json!({
        'A': {"A": [""], "^": ["<"], ">": ["v"], "v": ["<v"], "<": ["v<<"]},
        '^': {"^": [""], "A": [">"], "v": ["v"], "<": ["v<"], ">": ["v>"]},
        'v': {"v": [""], "A": [">^"], "^": ["^"], "<": ["<"], ">": [">"]},
        '<': {"<": [""], "A": [">>^"], "^": [">^"], "v": [">"], ">": [">>"]},
        '>': {">": [""], "A": ["^"], "^": ["<^"], "v": ["<"], "<": ["<<"]},
    });
    serde_json::from_value(value).unwrap()
}

pub fn travel_strs(from: char, to: char) -> Vec<String> {
    let from = RobotAction::from(from).get_pos();
    let to = RobotAction::from(to).get_pos();

    let (mut it, both_work) = move_to(from.0, from.1, to.0, to.1, 0, 0);
    let mut v = vec![it
        .clone()
        .into_iter()
        .flat_map(|(dir, num)| std::iter::repeat_n(char::from(dir), num))
        .collect::<String>()];
    if both_work {
        it.swap(0, 1);
        let new_str = it
            .clone()
            .into_iter()
            .flat_map(|(dir, num)| std::iter::repeat_n(char::from(dir), num))
            .collect();
        if (new_str != v[0]) {
            v.push(new_str);
        }
    }
    v
}

pub fn travel_strs_numpad(from: char, to: char) -> Vec<String> {
    let from = NUMPAD[from.to_digit(16).unwrap() as usize];
    let to = NUMPAD[to.to_digit(16).unwrap() as usize];

    let (mut it, both_work) = move_to(from.0, from.1, to.0, to.1, 0, 3);
    let mut v = vec![it
        .clone()
        .into_iter()
        .flat_map(|(dir, num)| std::iter::repeat_n(char::from(dir), num))
        .collect::<String>()];
    if both_work {
        it.swap(0, 1);
        let new_str = it
            .clone()
            .into_iter()
            .flat_map(|(dir, num)| std::iter::repeat_n(char::from(dir), num))
            .collect();
        if (new_str != v[0]) {
            v.push(new_str);
        }
    }
    v
}

#[rustfmt::skip]
const CHOOSE : [&[usize]; 7] = [
    &[choose(0, 0)],
    &[choose(1, 0), choose(1, 1)],
    &[choose(2, 0), choose(2, 2), choose(2, 2)],
    &[choose(3, 0), choose(3, 1), choose(3, 2), choose(3, 3)],
    &[choose(4, 0), choose(4, 1), choose(4, 2), choose(4, 3), choose(4, 4)],
    &[choose(5, 0), choose(5, 1), choose(5, 2), choose(5, 3), choose(5, 4), choose(5, 5)],
    &[choose(6, 0), choose(6, 1), choose(6, 2), choose(6, 3), choose(6, 4), choose(6, 5), choose(6, 6)],
];

const fn choose(n: usize, k: usize) -> usize {
    match () {
        _ if k > n => panic!("Invalid call to choose"),
        _ if k == 0 => 1,
        _ if k > n / 2 => choose(n, n - k),
        _ => n * choose(n - 1, k - 1) / k,
    }
}

const NUMPAD: [(usize, usize); 11] = [
    (1usize, 3usize),
    (0, 2),
    (1, 2),
    (2, 2),
    (0, 1),
    (1, 1),
    (2, 1),
    (0, 0),
    (1, 0),
    (2, 0),
    (2, 3),
];

fn num_from_pos(pos: (usize, usize)) -> char {
    char::from_digit(
        match pos {
            (1usize, 3usize) => 0,
            (0, 2) => 1,
            (1, 2) => 2,
            (2, 2) => 3,
            (0, 1) => 4,
            (1, 1) => 5,
            (2, 1) => 6,
            (0, 0) => 7,
            (1, 0) => 8,
            (2, 0) => 9,
            (2, 3) => 10,
            _ => panic!("invalid numpad position: {pos:?}"),
        },
        16,
    )
    .unwrap()
    .to_ascii_uppercase()
}

const NUMPAD_GAP_X: usize = 0;
const NUMPAD_GAP_Y: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RobotAction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Activate = 4,
}

impl RobotAction {
    pub fn get_pos(self) -> (usize, usize) {
        match self {
            Self::Up => (1, 0),
            Self::Down => (1, 1),
            Self::Left => (0, 1),
            Self::Right => (2, 1),
            Self::Activate => (2, 0),
        }
    }

    pub fn gap_x() -> usize {
        0
    }

    pub fn gap_y() -> usize {
        0
    }
}

impl From<char> for RobotAction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            'A' => Self::Activate,
            _ => panic!("invalid action: {value}"),
        }
    }
}

impl From<RobotAction> for char {
    fn from(value: RobotAction) -> Self {
        match value {
            RobotAction::Up => '^',
            RobotAction::Down => 'v',
            RobotAction::Left => '<',
            RobotAction::Right => '>',
            RobotAction::Activate => 'A',
        }
    }
}

impl From<(usize, usize)> for RobotAction {
    fn from(value: (usize, usize)) -> Self {
        match value {
            (1, 0) => Self::Up,
            (1, 1) => Self::Down,
            (0, 1) => Self::Left,
            (2, 1) => Self::Right,
            (2, 0) => Self::Activate,
            _ => panic!("invalid value: {value:?}"),
        }
    }
}

pub fn move_to(
    sx: usize,
    sy: usize,
    tx: usize,
    ty: usize,
    gap_x: usize,
    gap_y: usize,
) -> ([(RobotAction, usize); 2], bool) {
    let mut arr = [
        match sx.cmp(&tx) {
            Ordering::Less => (RobotAction::Right, sx.abs_diff(tx)),
            Ordering::Equal => (RobotAction::Right, 0),
            Ordering::Greater => (RobotAction::Left, sx.abs_diff(tx)),
        },
        match sy.cmp(&ty) {
            Ordering::Less => (RobotAction::Down, sy.abs_diff(ty)),
            Ordering::Equal => (RobotAction::Down, 0),
            Ordering::Greater => (RobotAction::Up, sy.abs_diff(ty)),
        },
    ];

    let vert_would_cross_gap = sy == gap_y && tx == gap_x;
    let horiz_would_cross_gap = ty == gap_y && sx == gap_x;

    // This is to prevent running into gaps
    // If the target is in the leftmost column (where both gaps are in the top and bottom corner)
    // then do the vertical movement first
    if vert_would_cross_gap {
        arr.swap(0, 1);
        (arr, false)
    } else {
        (arr, !vert_would_cross_gap && !horiz_would_cross_gap)
    }
}

pub const fn delta(sx: usize, sy: usize, tx: usize, ty: usize) -> (usize, usize) {
    (sx.abs_diff(tx), sy.abs_diff(ty))
}

pub const fn num_ways(sx: usize, sy: usize, tx: usize, ty: usize) -> usize {
    let (dx, dy) = delta(sx, sy, tx, ty);
    CHOOSE[dx][dx + dy]
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| (&line[..line.len() - 1], line))
        .collect_vec()
}
