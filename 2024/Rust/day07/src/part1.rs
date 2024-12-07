use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);
    let result: usize = input
        .into_iter()
        .map(|eq| {
            if solve_equation(eq.target, &eq.operands) > 0 {
                eq.target
            } else {
                0
            }
        })
        .sum();
    info!(result);
    Ok(())
}

fn solve_equation(target: usize, remaining: &[usize]) -> usize {
    let mut res = 0;
    let &current = remaining.last().unwrap();
    if remaining.len() == 1 {
        if current == target {
            return 1;
        } else {
            return 0;
        }
    }

    if target % current == 0 {
        res += solve_equation(target / current, &remaining[..remaining.len() - 1]);
    }

    res += solve_equation(target - current, &remaining[..remaining.len() - 1]);

    res
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
