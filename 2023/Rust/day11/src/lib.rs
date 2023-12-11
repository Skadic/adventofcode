use tracing::info;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str, scale: usize) -> (usize, usize, Vec<(usize, usize)>) {
    let _ = input;
    let mut galaxies = vec![];
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut y_offset = 0;
    for (row, line) in input.lines().enumerate() {
        let mut has_no_galaxy = true;
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((col, row + (y_offset * (scale - 1))));
                has_no_galaxy = false;
            }
        }
        if has_no_galaxy {
            y_offset += 1;
        }
    }

    galaxies.sort_unstable_by_key(|&(x, _)| x);

    let mut last_x = galaxies[0].0;
    let mut x_offset = 0;
    for pos @ &mut (x, y) in galaxies.iter_mut() {
        if x - last_x > 1 {
            x_offset += x - last_x - 1;
        }
        last_x = x;
        *pos = (x + (x_offset * (scale - 1)), y)
    }

    (width, height, galaxies)
}

pub fn find_distances(galaxies: &[(usize, usize)]) -> Vec<usize> {
    let mut v = vec![];
    for &(x1, y1) in galaxies.iter() {
        for &(x2, y2) in galaxies.iter() {
            v.push(x1.abs_diff(x2) + y1.abs_diff(y2));
        }
    }

    v
}
