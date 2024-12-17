use tracing::info;

use crate::{eval_input, make_input, parse_input, Opcode, INPUT, SAMPLE};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    eval_input(INPUT);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
