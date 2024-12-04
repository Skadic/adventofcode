use std::iter::{repeat, zip};

use aho_corasick::AhoCorasick;
use miette::IntoDiagnostic;
use tracing::info;

use crate::{parse_input, Grid, INPUT, TERMINATOR};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let grid = parse_input(INPUT);
    let texts = generate_texts(&grid);
    let ac = AhoCorasick::new(["XMAS"]).into_diagnostic()?;

    let result = ac.find_overlapping_iter(&texts).count();

    info!(result);
    Ok(())
}

pub fn generate_texts(grid: &Grid) -> String {
    let mut text = String::with_capacity(grid.width() * grid.height() * 8);
    // East
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            text.push(grid.get(x, y));
        }
        text.push(TERMINATOR);
    }
    text.push(TERMINATOR);

    // South
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            text.push(grid.get(x, y));
        }
        text.push(TERMINATOR);
    }
    text.push(TERMINATOR);

    // West
    for y in 0..grid.height() {
        for x in (0..grid.width()).rev() {
            text.push(grid.get(x, y));
        }
        text.push(TERMINATOR);
    }
    text.push(TERMINATOR);

    // North
    for x in 0..grid.width() {
        for y in (0..grid.height()).rev() {
            text.push(grid.get(x, y));
        }
        text.push(TERMINATOR);
    }
    text.push(TERMINATOR);

    // South East
    for (x, y) in zip(repeat(0), 0..grid.height()).chain(zip(1..grid.width(), repeat(0))) {
        zip(x..grid.width(), y..grid.height())
            .map(|(x, y)| grid.get(x, y))
            .for_each(|c| text.push(c));
        text.push(TERMINATOR);
    }
    text.push(TERMINATOR);

    // South West
    for (x, y) in
        zip(0..grid.width(), repeat(0)).chain(zip(repeat(grid.width() - 1), 1..grid.height()))
    {
        zip((0..=x).rev(), y..grid.height())
            .map(|(x, y)| grid.get(x, y))
            .for_each(|c| text.push(c));
        text.push(TERMINATOR);
    }
    text.push(TERMINATOR);

    // North West
    for (x, y) in zip(repeat(grid.width() - 1), 0..grid.height())
        .chain(zip((0..grid.width() - 1).rev(), repeat(grid.height() - 1)))
    {
        zip((0..=x).rev(), (0..=y).rev())
            .map(|(x, y)| grid.get(x, y))
            .for_each(|c| text.push(c));
        text.push(TERMINATOR);
    }
    text.push(TERMINATOR);

    // North East
    for (x, y) in zip((0..grid.width()).rev(), repeat(grid.height() - 1))
        .chain(zip(repeat(0), (0..grid.height() - 1).rev()))
    {
        zip(x..grid.width(), (0..=y).rev())
            .map(|(x, y)| grid.get(x, y))
            .for_each(|c| text.push(c));
        text.push(TERMINATOR);
    }

    text.push(TERMINATOR);
    text
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
