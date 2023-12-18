use tracing::info;

use crate::{ parse_input, INPUT, count4, path2};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let p = parse_input(INPUT);
    let (corners, _) = path2(&p);
    info!(result = count4(&corners));
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
