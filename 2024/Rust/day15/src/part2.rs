use tracing::info;

use crate::{parse_input, parse_input2, Slot, INPUT, SAMPLE, SAMPLE2};

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (mut grid, directions) = parse_input2(INPUT);

    println!("{grid}");

    for (i, dir) in directions.into_iter().enumerate() {
        grid.move_next(dir);
    }

    let mut result = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.0[y][x] == Slot::BoxL {
                result += y * 100 + x;
            }
        }
    }

    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
