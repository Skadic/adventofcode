use tracing::info;

use crate::{parse_input, Instruction, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let result = parse_input(INPUT)
        .into_iter()
        .scan(true, |enabled, instruction| {
            Some(match (*enabled, instruction) {
                (true, Instruction::Mul(l, r)) => l * r,
                (false, Instruction::Mul(_, _)) => 0,
                (_, Instruction::Do) => {
                    *enabled = true;
                    0
                }
                (_, Instruction::Dont) => {
                    *enabled = false;
                    0
                }
            })
        })
        .sum::<usize>();
    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
