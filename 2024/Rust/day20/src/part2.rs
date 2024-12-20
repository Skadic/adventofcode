use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;
use tracing::info;

use crate::{bfs, parse_input, Input, INPUT};

// 1243959 TOO HIGH

//#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let Input {
        map,
        start_x: sx,
        start_y: sy,
        ..
    } = parse_input(INPUT);
    const MAX_DIST: usize = 20;
    const MIN_ADVANTGE: usize = 100;

    let dists = bfs(&map, sx, sy);
    let dist_ref = &dists;

    let result = dists
        .keys()
        .flat_map(|&(x, y)| find_cheat_positions(&map, dist_ref, x, y, MAX_DIST))
        .filter(|&value| value >= MIN_ADVANTGE)
        .count();

    info!(result);

    Ok(())
}

fn find_cheat_positions<'a, 'b>(
    map: &'a [Vec<char>],
    dists: &'b HashMap<(usize, usize), usize>,
    x: usize,
    y: usize,
    max_dist: usize,
) -> impl IntoIterator<Item = usize> + Debug + use<'a, 'b> {
    let max_dist = max_dist as isize;
    Itertools::cartesian_product(-max_dist..=max_dist, -max_dist..=max_dist)
        .filter_map(move |(dx, dy)| Option::zip(x.checked_add_signed(dx), y.checked_add_signed(dy)))
        .filter(move |&(nx, ny)| {
            let dist_traveled = x.abs_diff(nx) + y.abs_diff(ny);
            (2..=max_dist as usize).contains(&dist_traveled)
        })
        .filter(|&(nx, ny)| ny < map.len() && nx < map[ny].len() && map[ny][nx] == '.')
        .filter_map(move |(nx, ny)| eval_cheat_value(dists, x, y, nx, ny))
}

fn eval_cheat_value(
    dists: &HashMap<(usize, usize), usize>,
    sx: usize,
    sy: usize,
    tx: usize,
    ty: usize,
) -> Option<usize> {
    dists[&(tx, ty)]
        .checked_sub(dists[&(sx, sy)] + sx.abs_diff(tx) + sy.abs_diff(ty))
        .filter(|&v| v > 0)
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}

#[cfg(test)]
#[test]
fn eval_cheat_value_test() {
    let hash = [((1, 3), 0), ((5, 2), 9)].into_iter().collect();
    assert_eq!(Some(4), eval_cheat_value(&hash, 1, 3, 5, 2));
}

#[cfg(test)]
#[test]
fn eval_cheat_value_test2() {
    let hash = [((1, 3), 9), ((5, 2), 0)].into_iter().collect();
    assert_eq!(None, eval_cheat_value(&hash, 1, 3, 5, 2));
}
