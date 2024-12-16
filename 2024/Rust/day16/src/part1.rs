use std::num::IntErrorKind;

use tracing::info;

use crate::{dijkstra, dijkstra1, parse_input, DistType, INPUT, SAMPLE};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);

    let dists = dijkstra1(&input, 1, input.len() - 2);

    let tup = dists[1][input[0].len()-2];
    info!(result = tup);

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
