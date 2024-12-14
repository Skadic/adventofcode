use std::cmp::Ordering;

use winnow::{
    ascii::{dec_int, line_ending, multispace0},
    combinator::separated,
    PResult, Parser,
};

pub const INPUT: (&str, usize, usize) = (include_str!("../input.txt"), 101, 103);
pub const SAMPLE: (&str, usize, usize) = (
    "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    11,
    7,
);

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Bot {
    pub x: isize,
    pub y: isize,
    pub dx: isize,
    pub dy: isize,
}

impl Bot {
    pub fn cycle_length(&self, w: usize, h: usize) -> usize {
        let w = w as isize;
        let h = h as isize;
        let mut current_x = (self.x + self.dx + w) % w;
        let mut x_cycle_len = 1;
        while current_x != self.x {
            current_x += self.dx + w;
            current_x %= w;
            x_cycle_len += 1;
        }

        let mut current_y = (self.y + self.dy + h) % h;
        let mut y_cycle_len = 1;
        while current_y != self.y {
            current_y += self.dy + h;
            current_y %= h;
            y_cycle_len += 1;
        }

        lcm(x_cycle_len, y_cycle_len)
    }

    pub fn move_once(&mut self, w: usize, h: usize) {
        let w = w as isize;
        let h = h as isize;

        self.x += self.dx + w;
        self.x %= w;
        self.y += self.dy + h;
        self.y %= h;
    }

    pub fn move_multiple(&mut self, w: usize, h: usize, n: usize) {
        let w = w as isize;
        let h = h as isize;
        let n = n as isize;

        self.x += n * (self.dx + w);
        self.x %= w;
        self.y += n * (self.dy + h);
        self.y %= h;
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Bot> {
    separated(1.., bot, line_ending).parse(input).unwrap()
}

pub fn bot(input: &mut &str) -> PResult<Bot> {
    winnow::combinator::seq! { Bot {
            _: "p=",
            x: dec_int,
            _: ",",
            y: dec_int,
            _: multispace0,
            _: "v=",
            dx: dec_int,
            _: ",",
            dy: dec_int,
        }
    }
    .parse_next(input)
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if a == 0 || b == 0 {
            return a.max(b);
        }
        match a.cmp(&b) {
            Ordering::Greater => a %= b,
            Ordering::Less => b %= a,
            Ordering::Equal => return a,
        }
    }
}

/// Least Common Multiple
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
