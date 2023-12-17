use crate::{parse_input, INPUT, dijkstra};
use tracing::info;

// 964 too high

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let map = parse_input(INPUT);

    info!(result = dijkstra(&map, 0, 0, 1, 3));
    Ok(())
}


#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
