use tracing::info;

use crate::{parse_input, Target, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let (workflows, parts) = parse_input(INPUT)?;

    let result = parts
        .into_iter()
        .filter(|part| {
            let mut current = &workflows["in"];
            while let Target::Workflow(next) = current.target(&part) {
                current = &workflows[next];
            }

            current.target(&part) == Target::Accept
        })
        .map(|v| v.value())
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
