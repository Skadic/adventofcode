use std::iter::repeat;

use tracing::info;

use crate::{parse_input, INPUT, Direction, track};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let map = parse_input(INPUT);
    // top
    let best = (0..map[0].len())
        .zip(repeat(0))
        .zip(repeat(Direction::Down))
        // left
        .chain(repeat(0).zip(0..map.len()).zip(repeat(Direction::Right)))
        //bottom
        .chain(
            (0..map[0].len())
                .zip(repeat(map.len() - 1))
                .zip(repeat(Direction::Up)),
        )
        //right
        .chain(
            repeat(map[0].len() - 1)
                .zip(0..map.len())
                .zip(repeat(Direction::Left)),
        )
        .map(|((x, y), dir)| track(&map, x, y, dir))
        .max()
        .unwrap();


    info!(result = best);
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
