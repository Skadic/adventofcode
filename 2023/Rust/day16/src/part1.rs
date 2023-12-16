
use tracing::info;

use crate::{parse_input, track, Direction, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let map = parse_input(INPUT);

    info!(result = track(&map, 0, 0, Direction::Right));
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
