use std::cell::RefCell;

use vers_vecs::{BitVec, RsVec};

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub mod part1;
pub mod part2;

pub struct Grid(Vec<Vec<char>>, QuickWalk);

impl Grid {
    pub fn get(&self, x: usize, y: usize) -> char {
        self.0[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize) {
        if self.get(x, y) == '#' {
            return;
        }

        self.0[y][x] = '#';
        self.1.remake_row(&self.0, y);
        self.1.remake_col(&self.0, x);
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        if self.get(x, y) != '#' {
            return;
        }

        self.0[y][x] = '.';
        self.1.remake_row(&self.0, y);
        self.1.remake_col(&self.0, x);
    }

    pub fn obstacle(&self, x: usize, y: usize) -> bool {
        self.get(x, y) == '#'
    }

    /// Give the next position if traveling in the current direction, if it's within the bounds of the grid
    pub fn next_pos(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => y.checked_sub(1).map(|y| (x, y)),
            Direction::Down => Some(y + 1).filter(|&y| y < self.height()).map(|y| (x, y)),
            Direction::Left => x.checked_sub(1).map(|x| (x, y)),
            Direction::Right => Some(x + 1).filter(|&x| x < self.width()).map(|x| (x, y)),
        }
    }

    pub fn width(&self) -> usize {
        self.0
            .first()
            .map(Vec::as_slice)
            .map(<[_]>::len)
            .unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn next_dir(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum AdvanceState {
    Forward,
    Turn,
    Border,
}

#[derive(Clone, Copy)]
struct Guard<'a> {
    direction: Direction,
    x: usize,
    y: usize,
    grid: &'a RefCell<Grid>,
}

impl<'a> Guard<'a> {
    pub fn new(grid: &'a RefCell<Grid>, x: usize, y: usize) -> Self {
        Self {
            direction: Direction::Up,
            x,
            y,
            grid,
        }
    }

    pub fn advance(&mut self, extra_obstacle: Option<(usize, usize)>) -> AdvanceState {
        let Some(next_pos @ (next_x, next_y)) = self.next_pos() else {
            return AdvanceState::Border;
        };

        if self.grid.borrow().obstacle(next_x, next_y)
            || extra_obstacle.map(|pos| next_pos == pos).unwrap_or(false)
        {
            self.direction = self.direction.next_dir();
            return AdvanceState::Turn;
        }

        self.x = next_x;
        self.y = next_y;
        AdvanceState::Forward
    }

    /// Give the next position if traveling in the current direction, if it's within the bounds of the grid
    pub fn next_pos(&self) -> Option<(usize, usize)> {
        match self.direction {
            Direction::Up => self.y.checked_sub(1).map(|y| (self.x, y)),
            Direction::Down => Some(self.y + 1)
                .filter(|&y| y < self.grid.borrow().height())
                .map(|y| (self.x, y)),
            Direction::Left => self.x.checked_sub(1).map(|x| (x, self.y)),
            Direction::Right => Some(self.x + 1)
                .filter(|&x| x < self.grid.borrow().width())
                .map(|x| (x, self.y)),
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn quick_walk(&mut self) -> bool {
        (self.x, self.y) = self
            .grid
            .borrow()
            .1
            .next_stop(self.x, self.y, self.direction);
        if self.next_pos().is_some() {
            self.direction = self.direction.next_dir();
            true
        } else {
            false
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> (Grid, (usize, usize)) {
    let mut pos = None;
    let raw_grid = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::chars)
        .map(|chars| chars.collect())
        .collect::<Vec<_>>();
    let quick_walk = QuickWalk::new(&raw_grid);
    let grid = Grid(raw_grid, quick_walk);

    'stuff: for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) == '^' {
                pos = Some((x, y));
                break 'stuff;
            }
        }
    }

    (grid, pos.unwrap())
}

struct QuickWalk {
    row_rs: Vec<RsVec>,
    col_rs: Vec<RsVec>,
}

impl QuickWalk {
    pub fn new(grid: &[Vec<char>]) -> Self {
        let row_rs = grid
            .iter()
            .map(|row| BitVec::from_bits_iter(row.iter().map(|&c| (c == '#') as u8)).into())
            .collect();
        let col_rs = (0..grid.len())
            .map(|x| {
                let col_bits = (0..grid[0].len()).map(|y| (grid[y][x] == '#') as u8);
                BitVec::from_bits_iter(col_bits).into()
            })
            .collect();

        Self { row_rs, col_rs }
    }

    pub fn remake_row(&mut self, grid: &[Vec<char>], y: usize) {
        let rs = BitVec::from_bits_iter(grid[y].iter().map(|&c| (c == '#') as u8)).into();
        self.row_rs[y] = rs;
    }

    pub fn remake_col(&mut self, grid: &[Vec<char>], x: usize) {
        let rs = BitVec::from_bits_iter((0..grid.len()).map(|y| (grid[y][x] == '#') as u8)).into();
        self.col_rs[x] = rs;
    }

    pub fn next_stop(&self, x: usize, y: usize, direction: Direction) -> (usize, usize) {
        match direction {
            Direction::Up => {
                let col_rs = &self.col_rs[x];
                let rank = col_rs.rank1(y);
                let new_y = if rank == 0 {
                    0
                } else {
                    col_rs.select1(rank - 1) + 1
                };
                (x, new_y)
            }
            Direction::Down => {
                let col_rs = &self.col_rs[x];
                let rank = col_rs.rank1(y);
                let new_y = col_rs.select1(rank) - 1;
                (x, new_y)
            }
            Direction::Left => {
                let row_rs = &self.row_rs[y];
                let rank = row_rs.rank1(x);
                let new_x = if rank == 0 {
                    0
                } else {
                    row_rs.select1(rank - 1) + 1
                };
                (new_x, y)
            }
            Direction::Right => {
                let row_rs = &self.row_rs[y];
                let rank = row_rs.rank1(x);
                let new_x = row_rs.select1(rank) - 1;
                (new_x, y)
            }
        }
    }
}
