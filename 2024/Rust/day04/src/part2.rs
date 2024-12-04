use std::iter::zip;

use aho_corasick::AhoCorasick;
use miette::IntoDiagnostic;
use tracing::info;

use crate::{parse_input, Grid, INPUT, TERMINATOR};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let grid = parse_input(INPUT);
    let text = generate_texts(&grid);
    let ac = AhoCorasick::new(["MASMAS"]).into_diagnostic()?;
    let result = ac.find_iter(&text).count();
    info!(result);
    Ok(())
}

#[tracing::instrument(skip_all, ret)]
pub fn generate_texts(grid: &Grid) -> String {
    let mut text = String::with_capacity(grid.width() * grid.height() * 4);

    for y in 0..grid.height() - 2 {
        for x in 0..grid.width() - 2 {
            text.push_str(&generate_x(grid, x, y));
        }
    }
    text
}

fn generate_x(grid: &Grid, x: usize, y: usize) -> String {
    let base_x = zip(x..x + 3, y..y + 3)
        .chain(zip(x..x + 3, (y..y + 3).rev()))
        .map(|(x, y)| grid.get(x, y))
        .collect::<Vec<char>>();

    let mut x = String::new();
    x.extend(&base_x);
    x.push(TERMINATOR);
    x.extend([0usize, 1, 2, 5, 4, 3].iter().map(|&i| base_x[i]));
    x.push(TERMINATOR);
    x.extend([5usize, 4, 3, 2, 1, 0].iter().map(|&i| base_x[i]));
    x.push(TERMINATOR);
    x.extend([2usize, 1, 0, 3, 4, 5].iter().map(|&i| base_x[i]));
    x.push(TERMINATOR);

    x
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
