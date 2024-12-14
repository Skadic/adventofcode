use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);
    let mut result = 0;

    for prize in input {
        let (target_x, target_y) = prize.prize;
        let (ax, ay) = prize.a;
        let (bx, by) = prize.b;
        let max_a_presses = ((target_x / ax).max(target_y / ay) + 1) as usize;
        let max_b_presses = ((target_x / bx).max(target_y / by) + 1) as usize;

        let delta = |a: usize, b: usize| -> (isize, isize) {
            let a = a as isize;
            let b = b as isize;
            (target_x - (a * ax + b * bx), target_y - (a * ay + b * by))
        };
        let tokens = |a: usize, b: usize| a * 3 + b;

        let check = |a: usize, b: usize| delta(a, b) == (0, 0);

        let mut arr = vec![];
        for a in 0..max_a_presses {
            for b in 0..max_b_presses {
                if check(a, b) {
                    arr.push((a, b));
                }
            }
        }

        result += arr
            .into_iter()
            .min()
            .map(|(a, b)| tokens(a, b))
            .unwrap_or(0);
    }

    // Edit here
    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
