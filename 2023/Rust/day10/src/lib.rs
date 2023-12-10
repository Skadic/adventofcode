use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};


pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
pub const SAMPLE2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
pub const SAMPLE3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

pub const SAMPLE4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Map {
    pub start_col: usize,
    pub start_row: usize,
    pub grid: Vec<Vec<char>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"")?;
        for row in self.grid.iter() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn apply(self, x: usize, y: usize) -> (usize, usize) {
        use Direction::*;
        match self {
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        }
    }
}

// FJL7-|.

impl Map {
    pub fn get(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    pub fn connects_to(&self, x: usize, y: usize, dir: Direction) -> bool {
        use Direction::*;
        let c = self.get(x, y);
        match dir {
            Up => "L|J".contains(c),
            Right => "L-F".contains(c),
            Down => "F|7".contains(c),
            Left => "J-7".contains(c),
        }
    }

    pub fn is_connected(&self, x: usize, y: usize, dir: Direction) -> bool {
        match dir {
            Direction::Up => {
                if y == 0 {
                    return false;
                }
                "F7|".contains(self.get(x, y - 1))
            }
            Direction::Right => {
                if x >= self.grid[y].len() - 1 {
                    return false;
                }
                "J7-".contains(self.get(x + 1, y))
            }
            Direction::Down => {
                if y >= self.grid.len() - 1 {
                    return false;
                }
                "JL|".contains(self.get(x, y + 1))
            }
            Direction::Left => {
                if x == 0 {
                    return false;
                }
                "LF-".contains(self.get(x - 1, y))
            }
        }
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    #[tracing::instrument(skip(self))]
    pub fn connected_neighbors(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
        use Direction::*;
        let mut res = [None; 4];
        for (i, d) in [Up, Right, Down, Left].into_iter().enumerate() {
            if self.is_connected(x, y, d) {
                res[i] = Some(d.apply(x, y))
            }
        }
        res
    }

    pub fn connected_neighbors_from(
        &self,
        x: usize,
        y: usize,
        src_dir: Direction,
    ) -> [Option<(usize, usize)>; 4] {
        use Direction::*;
        let mut res = [None; 4];
        for (i, d) in [Up, Right, Down, Left].into_iter().enumerate() {
            if d != src_dir && self.is_connected(x, y, d) {
                res[i] = Some(d.apply(x, y))
            }
        }
        res
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Map {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut m = Map {
        grid,
        start_col: 0,
        start_row: 0,
    };
    for row in 0..m.grid.len() {
        for col in 0..m.grid[0].len() {
            if m.grid[row][col] == 'S' {
                m.start_col = col;
                m.start_row = row;
                m.grid[row][col] = match m.connected_neighbors(col, row) {
                    [_, _, Some(_), Some(_)] => '7',
                    [_, Some(_), _, Some(_)] => '-',
                    [_, Some(_), Some(_), _] => 'F',
                    [Some(_), _, _, Some(_)] => 'J',
                    [Some(_), _, Some(_), _] => '|',
                    [Some(_), Some(_), _, _] => 'L',
                    _ => panic!("invalid combination of directions"),
                }
            }
        }
    }
    m
}

pub fn bfs(map: &Map) -> Vec<Vec<isize>> {
    let mut min_paths = vec![vec![-1isize; map.width()]; map.height()];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::<(usize, (usize, usize))>::new();
    queue.push_back((0, (map.start_col, map.start_row)));

    while !queue.is_empty() {
        let (current_len, pos @ (x, y)) = queue.pop_front().unwrap();
        visited.insert(pos);
        min_paths[y][x] = current_len as isize;

        use Direction::*;
        for (neighbor, dir) in map
            .connected_neighbors(x, y)
            .into_iter()
            .zip([Up, Right, Down, Left])
            .filter_map(|(v, dir)| v.map(|p| (p, dir)))
        {
            if visited.contains(&neighbor) || !map.connects_to(x, y, dir) {
                continue;
            }
            queue.push_back((current_len + 1, neighbor));
        }
    }

    min_paths
}
