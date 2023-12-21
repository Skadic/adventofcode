use tracing::info;

use crate::{bfs_part1, parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let (m, x, y) = parse_input(INPUT);
    info!(
        result = bfs_part1(&m, x, y, 64)
            .into_iter()
            .flat_map(|c| c)
            .filter(|&b| b)
            .count()
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
