use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(match s.to_lowercase().as_str() {
            "l" => Left,
            "r" => Right,
            "u" => Up,
            "d" => Down,
            _ => return Err(format!("invalid direction '{s}'")),
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Rope<const LENGTH: usize> {
    pub knots: [(isize, isize); LENGTH],
}

impl<const LENGTH: usize> Rope<LENGTH> {
    #[inline]
    pub fn new() -> Self {
        Self {
            knots: [(0, 0); LENGTH],
        }
    }

    pub fn tail(&self) -> (isize, isize) {
        self.knots[self.knots.len() - 1]
    }

    pub fn index(&self, pos: (isize, isize)) -> Option<usize> {
        self.knots.iter().position(|&p| p == pos)
    }

    pub fn tug(&mut self, direction: Direction) {
        use Direction::*;

        match direction {
            Left => {
                self.knots[0].0 -= 1;
            }
            Right => {
                self.knots[0].0 += 1;
            }
            Up => {
                self.knots[0].1 -= 1;
            }
            Down => {
                self.knots[0].1 += 1;
            }
        }

        // We need an extended moveset now.
        // Horizontal movement always before vertical
        let mut direction = match direction {
            Left => "l",
            Right => "r",
            Up => "u",
            Down => "d",
        }
        .to_owned();

        for i in 1..LENGTH {
            let (predecessor_x, predecessor_y) = self.knots[i - 1];
            let (x, y) = &mut self.knots[i];
            let new_direction = match direction.as_str() {
                "l" => {
                    if predecessor_x.abs_diff(*x) > 1 {
                        *x -= 1;
                        if predecessor_y != *y {
                            let difference = predecessor_y - *y;
                            *y += difference;
                            format!("l{}", if difference < 0 { 'u' } else { 'd' })
                        } else {
                            "l".into()
                        }
                    } else {
                        return;
                    }
                }
                "r" => {
                    if predecessor_x.abs_diff(*x) > 1 {
                        *x += 1;
                        if predecessor_y != *y {
                            let difference = predecessor_y - *y;
                            *y += difference;
                            format!("r{}", if difference < 0 { 'u' } else { 'd' })
                        } else {
                            "r".into()
                        }
                    } else {
                        return;
                    }
                }
                "u" => {
                    if predecessor_y.abs_diff(*y) > 1 {
                        *y -= 1;
                        if predecessor_x != *x {
                            let difference = predecessor_x - *x;
                            *x += difference;
                            format!("u{}", if difference < 0 { 'l' } else { 'r' })
                        } else {
                            "u".into()
                        }
                    } else {
                        return;
                    }
                }
                "d" => {
                    if predecessor_y.abs_diff(*y) > 1 {
                        *y += 1;
                        if predecessor_x != *x {
                            let difference = predecessor_x - *x;
                            *x += difference;
                            format!("d{}", if difference < 0 { 'l' } else { 'r' })
                        } else {
                            "d".into()
                        }
                    } else {
                        return;
                    }
                }
                s if s.len() > 1 => {
                    if predecessor_x.abs_diff(*x) > 1 || predecessor_y.abs_diff(*y) > 1 {
                        let mut chars = s.chars();

                        let h_move;
                        let v_move;

                        match chars.next().unwrap() {
                            c if c == 'l' || c == 'r' => {
                                h_move = c;
                                v_move = chars.next().unwrap()
                            }
                            c => {
                                v_move = c;
                                h_move = chars.next().unwrap()
                            }
                        }

                        if predecessor_x.abs_diff(*x) == 2 {
                            match h_move {
                                'l' => *x -= 1,
                                'r' => *x += 1,
                                _ => panic!(),
                            };
                            if predecessor_y.abs_diff(*y) > 0 {
                                match v_move {
                                    'u' => *y -= 1,
                                    'd' => *y += 1,
                                    _ => panic!(),
                                };
                                s.to_owned()
                            } else {
                                h_move.to_string()
                            }
                        } else {
                            match v_move {
                                'u' => *y -= 1,
                                'd' => *y += 1,
                                _ => panic!(),
                            };
                            if predecessor_x.abs_diff(*x) > 0 {
                                match h_move {
                                    'l' => *x -= 1,
                                    'r' => *x += 1,
                                    _ => panic!(),
                                };
                                s.to_owned()
                            } else {
                                v_move.to_string()
                            }
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    panic!("Invalid direction: {direction}")
                }
            };

            //print!("{i} moving {}, ", &new_direction);
            direction = new_direction;
        }
    }
}

impl<const LENGTH: usize> Default for Rope<LENGTH> {
    fn default() -> Self {
        Self::new()
    }
}

fn process_input(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(str::trim)
        .flat_map(|line| {
            let mut split = line.split_whitespace();
            let direction = split.next().unwrap().parse::<Direction>().unwrap();
            let amount = split.next().unwrap().parse::<usize>().unwrap();
            std::iter::repeat(direction).take(amount)
        })
        .collect()
}

pub fn process_part1(input: &str) -> usize {
    let directives = process_input(input);
    let mut rope = Rope::<2>::new();

    let mut positions = HashSet::new();
    let mut previous_tail_position = rope.tail();

    for direction in directives {
        rope.tug(direction);
        let min_x = rope.tail().0.min(previous_tail_position.0);
        let max_x = rope.tail().0.max(previous_tail_position.0);
        let min_y = rope.tail().1.min(previous_tail_position.1);
        let max_y = rope.tail().1.max(previous_tail_position.1);
        previous_tail_position = rope.tail();

        use Direction::*;
        match direction {
            Left | Right => (min_x + 1..max_x)
                .zip(std::iter::repeat(previous_tail_position.1))
                .chain(std::iter::once(rope.tail()))
                .for_each(|pos| {
                    positions.insert(pos);
                }),

            Up | Down => std::iter::repeat(previous_tail_position.0)
                .zip(min_y + 1..max_y)
                .chain(std::iter::once(rope.tail()))
                .for_each(|pos| {
                    positions.insert(pos);
                }),
        };
    }

    positions.len()
}

pub fn process_part2(input: &str) -> usize {
    let directives = process_input(input);
    let mut rope = Rope::<10>::new();

    let mut positions = HashSet::new();
    let mut previous_tail_position = rope.tail();

    let mut i = 0;
    for direction in directives {
        rope.tug(direction);
        let min_x = rope.tail().0.min(previous_tail_position.0);
        let max_x = rope.tail().0.max(previous_tail_position.0);
        let min_y = rope.tail().1.min(previous_tail_position.1);
        let max_y = rope.tail().1.max(previous_tail_position.1);
        previous_tail_position = rope.tail();

        use Direction::*;
        match direction {
            Left | Right => (min_x + 1..max_x)
                .zip(std::iter::repeat(previous_tail_position.1))
                .chain(std::iter::once(rope.tail()))
                .for_each(|pos| {
                    positions.insert(pos);
                }),

            Up | Down => std::iter::repeat(previous_tail_position.0)
                .zip(min_y + 1..max_y)
                .chain(std::iter::once(rope.tail()))
                .for_each(|pos| {
                    positions.insert(pos);
                }),
        };
    }

    positions.len()
}

pub fn print_positions<const LENGTH: usize>(
    rope: Rope<LENGTH>,
    positions: &HashSet<(isize, isize)>,
) {
    let (min_x, max_x, min_y, max_y) = positions
        .iter()
        .copied()
        .chain(rope.knots.into_iter())
        .fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(old_min_x, old_max_x, old_min_y, old_max_y), (x, y)| {
                (
                    old_min_x.min(x),
                    old_max_x.max(x),
                    old_min_y.min(y),
                    old_max_y.max(y),
                )
            },
        );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (x, y) == (0, 0) {
                print!("$");
                continue;
            }
            if let Some(i) = rope.index((x, y)) {
                if i == 0 {
                    print!("H");
                    continue;
                }
                print!("{i}");
                continue;
            }
            if positions.contains(&(x, y)) {
                print!("#");
                continue;
            }

            print!(".");
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod test {
    use crate::{process_part1, process_part2};

    #[test]
    fn test_part1() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(13, process_part1(input));
    }

    #[test]
    fn test_part2_2() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(36, process_part2(input));
    }

    #[test]
    fn test_part2_1() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(1, process_part2(input));
    }
}
