use tracing::info;

use crate::{parse_input, path, INPUT, count};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let p = parse_input(INPUT);
    let a = path(&p);

    info!(result = count(&a.1));
    Ok(())
}


#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
