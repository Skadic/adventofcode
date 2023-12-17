use tracing::info;

use crate::{parse_input, INPUT, dijkstra};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let map = parse_input(INPUT);
    info!(result = dijkstra(&map, 0, 0, 4, 10));
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
