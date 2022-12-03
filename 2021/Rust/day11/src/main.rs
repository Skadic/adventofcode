use std::collections::{HashSet, VecDeque};

fn main() {
    let grid = include_str!("../res/input.txt")
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    println!("Part 1: {}", part1(grid.clone(), 100));
    println!("Part 2: {}", part2(grid));
}

fn part1(mut grid: Vec<Vec<usize>>, steps: usize) -> usize {
    let mut flash_count = 0;

    for _ in 0..steps {
        flash_count += step(&mut grid);
    }
    flash_count
}

fn part2(mut grid: Vec<Vec<usize>>) -> usize {

    let mut i = 1;
    loop {
        if step(&mut grid) == grid.len() * grid[0].len() {
            return i;
        }
        i += 1;
    }
}

fn step(mut grid: &mut [Vec<usize>]) -> usize {
    let mut flash_count = 0;
    grid.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|octopus| *octopus += 1);
        let mut flash_required = true;

        while flash_required {
            flash_required = false;
            let mut visited = HashSet::new();
            let mut flashed = HashSet::new();
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    let octopus = grid[y][x];
                    if octopus > 9 && !visited.contains(&(x, y)) {
                        flash_required = true;
                        flash(&mut grid, &mut visited, &mut flashed, x, y);
                    }
                }
            }

            flash_count += visited.len();

            for (x, y) in visited.into_iter() {
                grid[y][x] = 0;
            }
        }
        flash_count
}

fn flash(
    grid: &mut [Vec<usize>],
    visited: &mut HashSet<(usize, usize)>,
    flashed: &mut HashSet<(usize, usize)>,
    start_x: usize,
    start_y: usize,
) -> usize {
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let neighbors = |x: usize, y: usize| {
        [-1, 0, 1]
            .into_iter()
            .flat_map(|i| [-1, 0, 1].into_iter().map(move |j| (i, j)))
            .filter(|&(x, y)| x != 0 || y != 0)
            .map(|(x_offset, y_offset)| (((x as isize) + x_offset) as usize, ((y as isize) + y_offset) as usize))
            .filter(|&(x, y)| x < grid_width && y < grid_height)
            .collect::<Vec<(usize, usize)>>()
    };

    let mut flash_count = 0;
    let mut must_flash = VecDeque::from_iter(std::iter::once((start_x, start_y)));
    visited.insert((start_x, start_y));
    while let Some(pos @ (x, y)) = must_flash.pop_front() {
        if flashed.contains(&pos) { continue; }
        flash_count += 1;
        flashed.insert(pos);
        for neighbor_pos @ (n_x, n_y) in neighbors(x, y) {
            let neighbor_elem = grid[n_y].get_mut(n_x).unwrap();
            *neighbor_elem += 1;
            if *neighbor_elem > 9 && !visited.contains(&neighbor_pos) {
                visited.insert(neighbor_pos);
                must_flash.push_back(neighbor_pos);
            }
        }
    }

    flash_count
}