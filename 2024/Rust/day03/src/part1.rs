use tracing::info;

use crate::{parse_input, Instruction, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let result = parse_input(INPUT)
        .into_iter()
        .filter_map(|v| {
            if let Instruction::Mul(l, r) = v {
                Some(l * r)
            } else {
                None
            }
        })
        .sum::<usize>();
    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
