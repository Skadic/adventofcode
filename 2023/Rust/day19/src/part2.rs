use std::sync::atomic::AtomicUsize;

use tracing::info;

use crate::{parse_input, PartRange, Target, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (workflows, _) = parse_input(INPUT)?;

    let inflow = &workflows["in"];

    let start = PartRange::default();
    let mut current = vec![(start, inflow)];
    let sum = AtomicUsize::new(0);

    while !current.is_empty() {
        let taken = std::mem::take(&mut current);
        current = taken
            .into_iter()
            .flat_map(|(old_rng, workflow)| {
                workflow
                    .with_range(old_rng)
                    .filter_map(|(rng, target)| match target {
                        Target::Accept => {
                            sum.fetch_add(rng.possibilities(), std::sync::atomic::Ordering::SeqCst);
                            None
                        }
                        Target::Reject => None,
                        Target::Workflow(next) => Some((rng, &workflows[next])),
                    })
            })
            .collect();

    }

    info!(result = sum.load(std::sync::atomic::Ordering::SeqCst));
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
