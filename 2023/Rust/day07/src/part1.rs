use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let mut plays = parse_input(INPUT);
    plays.sort_unstable_by(|l, r| l.cmp_lex(r));

    let result = plays
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i + 1) * v.bid);

    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
