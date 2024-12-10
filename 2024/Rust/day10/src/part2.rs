use tracing::info;

use crate::{dfs_trails2, parse_input, INPUT, SAMPLE};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);
    let result = dfs_trails2(&input);

    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
