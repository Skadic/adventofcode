use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let mut plays = parse_input(INPUT);
    plays.sort_unstable_by(|l, r| l.cmp_2nd(r));

    let result = plays
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i + 1) * v.bid);

    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}

// 253251722 too low
// 253907829
