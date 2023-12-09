use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let histories = parse_input(INPUT);

    info!(result = histories.into_iter().map(|v| history(&v)).sum::<isize>());
    Ok(())
}

fn history(hist: &[isize]) -> isize {
    let mut hist = hist.to_vec();
    let mut last_vals = Vec::with_capacity(hist.len());
    let mut done = false;
    last_vals.push(*hist.last().unwrap());
    while !done {
        done = true;
        for i in 0..hist.len() - 1 {
            hist[i] = hist[i + 1] - hist[i];
            if hist[i] != 0 {
                done = false;
            }
        }
        hist.pop();
        last_vals.push(*hist.last().unwrap())
    }
    last_vals.into_iter().sum()
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
