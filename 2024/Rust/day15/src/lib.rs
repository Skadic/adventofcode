use std::{borrow::Borrow, fmt::Display};

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

pub const SAMPLE2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

pub mod part1;
pub mod part2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Slot {
    Wall,
    Boxx,
    Empty,
    BoxL,
    BoxR,
}

impl Slot {
    fn is_box(self) -> bool {
        matches!(self, Self::Boxx | Self::BoxL | Self::BoxR)
    }
}

impl From<char> for Slot {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' | '@' => Self::Empty,
            'O' => Self::Boxx,
            '[' => Self::BoxL,
            ']' => Self::BoxR,
            _ => panic!("invalid slot char: {value}"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grid(Vec<Vec<Slot>>, Bot);

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.1.x == x && self.1.y == y {
                    write!(f, "@")?;
                } else {
                    write!(
                        f,
                        "{}",
                        match self.0[y][x] {
                            Slot::Wall => '#',
                            Slot::Empty => '.',
                            Slot::Boxx => 'O',
                            Slot::BoxL => '[',
                            Slot::BoxR => ']',
                        }
                    )?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("invalid direction char: {value}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bot {
    pub x: usize,
    pub y: usize,
}

impl Grid {
    pub fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn move_next(&mut self, direction: Direction) -> Option<(usize, usize)> {
        let bot = &self.1;
        let (x, y) = self.next_pos(bot.x, bot.y, direction)?;

        match self.0[y][x] {
            Slot::Wall => return None,
            Slot::Empty => {}
            Slot::Boxx => {
                let mut box_x = x;
                let mut box_y = y;
                while let Some((next_x, next_y)) = self.next_pos(box_x, box_y, direction) {
                    match self.0[next_y][next_x] {
                        Slot::Boxx => {
                            box_x = next_x;
                            box_y = next_y;
                        }
                        Slot::Wall => return None,
                        Slot::Empty => {
                            self.0[next_y][next_x] = Slot::Boxx;
                            self.0[y][x] = Slot::Empty;
                            break;
                        }
                        Slot::BoxL | Slot::BoxR => {}
                    }
                }
            }
            Slot::BoxL | Slot::BoxR => {
                if !self.can_move_box(x, y, direction) {
                    return None;
                } else {
                    self.move_box(x, y, direction);
                }
            }
        }

        let bot = &mut self.1;
        bot.x = x;
        bot.y = y;

        Some((x, y))
    }

    fn next_pos(&self, mut x: usize, mut y: usize, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => y = y.checked_sub(1)?,
            Direction::Down => y = y.checked_add(1).filter(|&y| y < self.height())?,
            Direction::Left => x = x.checked_sub(1)?,
            Direction::Right => x = x.checked_add(1).filter(|&x| x < self.width())?,
        }

        Some((x, y))
    }

    #[tracing::instrument(skip(self))]
    fn can_move_box(&self, x: usize, y: usize, direction: Direction) -> bool {
        if self.0[y][x] == Slot::Empty {
            return true;
        }
        match direction {
            Direction::Left | Direction::Right => {
                let (next_x, next_y) = self
                    .next_pos(x, y, direction)
                    .and_then(|(x, y)| self.next_pos(x, y, direction))
                    .unwrap();
                let next = self.0[next_y][next_x];
                match next {
                    Slot::BoxL | Slot::BoxR | Slot::Boxx
                        if self.can_move_box(next_x, next_y, direction) =>
                    {
                        true
                    }
                    Slot::BoxL | Slot::BoxR | Slot::Boxx => false,
                    Slot::Empty => true,
                    Slot::Wall => false,
                }
            }
            Direction::Up | Direction::Down => {
                let (_, next_y) = self.next_pos(x, y, direction).unwrap();
                let is_left = self.0[y][x] == Slot::BoxL;
                let other_x = if is_left { x + 1 } else { x - 1 };
                let next = self.0[next_y][x];
                let other_next = self.0[next_y][other_x];
                if next == Slot::Wall || other_next == Slot::Wall {
                    return false;
                }
                (next == Slot::Empty || self.can_move_box(x, next_y, direction))
                    && (other_next == Slot::Empty || self.can_move_box(other_x, next_y, direction))
            }
        }
    }

    fn move_box(&mut self, x: usize, y: usize, direction: Direction) -> bool {
        if self.0[y][x] == Slot::Empty {
            return true;
        }
        match direction {
            Direction::Left | Direction::Right => {
                let (next_x, next_y) = self
                    .next_pos(x, y, direction)
                    .and_then(|(x, y)| self.next_pos(x, y, direction))
                    .unwrap();
                let next = self.0[next_y][next_x];
                match next {
                    Slot::BoxL | Slot::BoxR | Slot::Boxx
                        if self.move_box(next_x, next_y, direction) =>
                    {
                        self.0[y][x] = Slot::Empty;
                        if direction == Direction::Left {
                            self.0[y][x - 1] = Slot::BoxR;
                            self.0[y][x - 2] = Slot::BoxL;
                        } else {
                            self.0[y][x + 1] = Slot::BoxL;
                            self.0[y][x + 2] = Slot::BoxR;
                        }
                        true
                    }
                    Slot::BoxL | Slot::BoxR | Slot::Boxx => false,
                    Slot::Empty => {
                        self.0[y][x] = Slot::Empty;
                        if direction == Direction::Left {
                            self.0[y][x - 1] = Slot::BoxR;
                            self.0[y][x - 2] = Slot::BoxL;
                        } else {
                            self.0[y][x + 1] = Slot::BoxL;
                            self.0[y][x + 2] = Slot::BoxR;
                        }
                        true
                    }
                    Slot::Wall => false,
                }
            }
            Direction::Up | Direction::Down => {
                let (_, next_y) = self.next_pos(x, y, direction).unwrap();
                let is_left = self.0[y][x] == Slot::BoxL;
                let other_x = if is_left { x + 1 } else { x - 1 };
                let next = self.0[next_y][x];
                if next.is_box() && !self.can_move_box(x, next_y, direction) {
                    return false;
                }
                let other_next = self.0[next_y][other_x];
                if other_next.is_box() && !self.can_move_box(other_x, next_y, direction) {
                    return false;
                }
                self.move_box(x, next_y, direction);
                self.move_box(other_x, next_y, direction);

                self.0[y][x] = Slot::Empty;
                if is_left {
                    self.0[y][x + 1] = Slot::Empty;
                    self.0[next_y][x] = Slot::BoxL;
                    self.0[next_y][x + 1] = Slot::BoxR;
                } else {
                    self.0[y][x - 1] = Slot::Empty;
                    self.0[next_y][x - 1] = Slot::BoxL;
                    self.0[next_y][x] = Slot::BoxR;
                }
                true
            }
        }
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> (Grid, Vec<Direction>) {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let data: Vec<Vec<_>> = map.lines().map(|line| line.chars().collect()).collect();

    let mut bot = None;
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] == '@' {
                bot = Some(Bot { x, y });
                break;
            }
        }
    }

    (
        Grid(
            data.into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
            bot.unwrap(),
        ),
        directions
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Into::into)
            .collect(),
    )
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input2(input: &str) -> (Grid, Vec<Direction>) {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let data: Vec<Vec<_>> = map.lines().map(|line| line.chars().collect()).collect();

    let mut bot = None;
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] == '@' {
                bot = Some(Bot { x: 2 * x, y });
                break;
            }
        }
    }

    (
        Grid(
            data.into_iter()
                .map(|row| {
                    row.into_iter()
                        .flat_map(|c| {
                            if c == 'O' {
                                [Slot::BoxL, Slot::BoxR]
                            } else {
                                [c.into(), c.into()]
                            }
                        })
                        .collect()
                })
                .collect(),
            bot.unwrap(),
        ),
        directions
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Into::into)
            .collect(),
    )
}
