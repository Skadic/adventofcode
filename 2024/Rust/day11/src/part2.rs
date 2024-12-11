use std::collections::HashMap;

use tracing::info;

use crate::{parse_input, step, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let stones = parse_input(INPUT);
    let mut stones = stones.into_iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    for _ in 0..75 {
        step(&mut stones);
    }
    let result: usize = stones.values().sum();
    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
