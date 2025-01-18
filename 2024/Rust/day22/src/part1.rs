use tracing::info;

use crate::{nth_secret_number, parse_input, INPUT};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);
    let result = input.into_iter().map(|n| nth_secret_number(n, 2000)).sum::<usize>();
    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
