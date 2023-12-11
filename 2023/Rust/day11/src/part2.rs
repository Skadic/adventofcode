use tracing::info;

use crate::{find_distances, parse_input, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (_, _, galaxies) = parse_input(INPUT, 1000000);
    info!(result = find_distances(&galaxies).into_iter().sum::<usize>() / 2);
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
