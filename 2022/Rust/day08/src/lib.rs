use std::ops::Deref;

use Direction::*;

pub struct Grid(Vec<Vec<usize>>);

impl Deref for Grid {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Grid {
    fn visible(&self, x: usize, y: usize) -> bool {
        !self.visible_directions(x, y).is_empty()
    }

    fn visible_directions(&self, x: usize, y: usize) -> Vec<Direction> {
        [Left, Right, Up, Down]
            .into_iter()
            .filter(|&dir| self.visible_in_direction(x, y, dir))
            .collect()
    }

    fn visible_in_direction(&self, x_pos: usize, y_pos: usize, dir: Direction) -> bool {
        let w = self.width();
        let h = self.height();
        let tree_height = self[y_pos][x_pos];
        || -> Box<dyn Iterator<Item = _>> {
            match dir {
                Left => Box::new((0..x_pos).zip(std::iter::repeat(y_pos))),
                Right => Box::new((x_pos + 1..w).zip(std::iter::repeat(y_pos))),
                Up => Box::new(std::iter::repeat(x_pos).zip(0..y_pos)),
                Down => Box::new(std::iter::repeat(x_pos).zip(y_pos + 1..h)),
            }
        }()
        .map(|(x, y)| self[y][x])
        .all(|height| height < tree_height)
    }

    fn view_distance_in_direction(&self, x: usize, y: usize, dir: Direction) -> usize {
        let w = self.width();
        let h = self.height();
        let tree_height = self[y][x];
        if let Some(distance) = || -> Box<dyn Iterator<Item = _>> {
            match dir {
                Left => Box::new((0..x).rev().zip(std::iter::repeat(y))),
                Right => Box::new((x + 1..w).zip(std::iter::repeat(y))),
                Up => Box::new(std::iter::repeat(x).zip((0..y).rev())),
                Down => Box::new(std::iter::repeat(x).zip(y + 1..h)),
            }
        }()
        .enumerate()
        .map(|(i, (x, y))| (i, self[y][x]))
        .position(|(i, height)| height >= tree_height)
        {
            return distance + 1;
        };

        match dir {
            Left => x,
            Right => w - x - 1,
            Up => y,
            Down => h - y - 1,
        }
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        [Left, Right, Up, Down]
            .into_iter()
            .map(|dir| self.view_distance_in_direction(x, y, dir))
            .product()
    }

    fn width(&self) -> usize {
        self.get(0).map(Vec::len).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.len()
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn process_input(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
    )
}

pub fn process_part1(input: &str) -> usize {
    let grid = process_input(input);
    let w = grid[0].len();
    let h = grid.len();

    (0..w)
        .flat_map(|i| std::iter::repeat(i).zip(0..h))
        .filter(|&(x, y)| grid.visible(x, y))
        .count()
}

pub fn process_part2(input: &str) -> usize {
    let grid = process_input(input);
    let w = grid[0].len();
    let h = grid.len();

    (0..w)
        .flat_map(|i| std::iter::repeat(i).zip(0..h))
        .map(|(x, y)| grid.scenic_score(x, y))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{process_part1, process_part2};
    const INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_part1() {
        assert_eq!(21, process_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, process_part2(INPUT));
    }
}
