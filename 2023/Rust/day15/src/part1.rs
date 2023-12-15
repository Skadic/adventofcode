use tracing::info;

use crate::INPUT;

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    info!(
        result = INPUT
            .split(',')
            .map(|s| s
                .trim()
                .bytes()
                .fold(0, |acc, c| ((acc + c as usize) * 17) % 256))
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
