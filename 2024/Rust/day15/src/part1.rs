use tracing::info;

use crate::{parse_input, Grid, Slot, INPUT, SAMPLE, SAMPLE2};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let (mut grid, directions) = parse_input(INPUT);

    for dir in directions.into_iter() {
        grid.move_next(dir);
    }

    let mut result = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.0[y][x] == Slot::Boxx {
                result += y * 100 + x;
            }
        }
    }

    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
