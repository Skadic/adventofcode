use tracing::info;

use crate::{dfs_trails, parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);
    let result = dfs_trails(&input);

    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
