use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let histories = parse_input(INPUT);

    info!(result = histories.into_iter().map(|v| history(&v)).sum::<isize>());
    Ok(())
}

fn history(hist: &[isize]) -> isize {
    let mut hist = hist.to_vec();
    let mut first_vals = Vec::with_capacity(hist.len());
    let mut done = false;
    first_vals.push(*hist.first().unwrap());
    while !done {
        done = true;
        for i in 0..hist.len() - 1 {
            hist[i] = hist[i + 1] - hist[i];
            if hist[i] != 0 {
                done = false;
            }
        }
        hist.pop();
        first_vals.push(*hist.first().unwrap())
    }
    let mut a = 0;
    for v in first_vals.into_iter().rev() {
        a = v - a;
    }
    a
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}

// TOO HIGH 19573
