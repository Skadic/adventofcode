use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let (rl, _, dirs) = parse_input(INPUT);
    let mut iter = rl.iter();

    let mut current = "AAA";
    let mut steps = 0;

    while current != "ZZZ" {
        if let Some(&next) = iter.next() {
            current = dirs[current][next];
            steps += 1;
        } else {
            iter = rl.iter();
        }
    }

    info!(result = steps);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
