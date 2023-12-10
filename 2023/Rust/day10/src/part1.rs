use tracing::info;

use crate::{parse_input, INPUT, bfs};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let map = parse_input(INPUT);
    
    info!(result = bfs(&map).into_iter().flat_map(|v| v.into_iter()).max().unwrap());
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
