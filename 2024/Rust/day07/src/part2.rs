use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);
    let result: usize = input
        .into_iter()
        .map(|eq| {
            if solve_equation(eq.target, &eq.operands[1..], eq.operands[0]) {
                eq.target
            } else {
                0
            }
        })
        .sum();
    info!(result);
    Ok(())
}

fn solve_equation(target: usize, remaining: &[usize], acc: usize) -> bool {
    if remaining.is_empty() {
        return acc == target;
    }

    if solve_equation(target, &remaining[1..], concat(acc, remaining[0])) {
        return true;
    }

    if solve_equation(target, &remaining[1..], acc + remaining[0]) {
        return true;
    }

    if solve_equation(target, &remaining[1..], acc * remaining[0]) {
        return true;
    }

    return false;
}

fn concat(l: usize, r: usize) -> usize {
    let r_digits = ((r + 1) as f64).log10().ceil() as u32;
    l * 10usize.pow(r_digits) + r
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
