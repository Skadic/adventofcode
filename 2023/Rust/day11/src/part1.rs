use tracing::info;

use crate::{parse_input, find_distances, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let (_, _, galaxies) = parse_input(INPUT, 2);
    info!(result = find_distances(&galaxies).into_iter().sum::<usize>() / 2);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
