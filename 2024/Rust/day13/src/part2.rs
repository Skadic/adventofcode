use std::isize;

use tracing::{info, warn};

use crate::{parse_input, Prize, INPUT};
use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let mut input = parse_input(INPUT);
    for p in input.iter_mut() {
        p.prize.0 += 10000000000000;
        p.prize.1 += 10000000000000;
    }

    // It's past 3am

    let mut result = 0;
    for prize in input.iter() {
        let solution = all_solutions(*prize);
        result += solution
            .map(|(a, b)| a * 3 + b)
            .unwrap_or(0);
    }
    info!(result);

    Ok(())
}

pub fn all_solutions(prize: Prize) -> Option<(isize, isize)> {
    // ax a + bx b = px
    // ay a + by b = py

    let (ax, ay) = prize.a;
    let (bx, by) = prize.b;
    let (prize_x, prize_y) = prize.prize;
    let arr: Array2<f64> = array![[ax as f64, bx as f64], [ay as f64, by as f64]];
    let target: Array1<f64> = array![prize_x as f64, prize_y as f64];
    arr.solve_into(target).ok().and_then(|v| {
        let rounded = (v[0].round() as isize, v[1].round() as isize);
        if (v[0].abs() - (rounded.0 as f64).abs()).abs() > 0.0001
            || (v[1].abs() - (rounded.1 as f64).abs()).abs() > 0.0001
        {
            None
        } else {
            Some(rounded)
        }
    })
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
