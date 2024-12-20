use std::collections::HashMap;

use itertools::Itertools;
use tracing::info;

use crate::{bfs, parse_input, Input, INPUT};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let Input {
        map,
        start_x: sx,
        start_y: sy,
        ..
    } = parse_input(INPUT);

    let dists = bfs(&map, sx, sy);

    let result = dists
        .iter()
        .flat_map(|(&sp @ (x, y), _)| {
            find_cheat_positions(&map, x, y).into_iter().map(move |tp: (usize, usize)| (sp, tp))
        })
        .filter_map(|(sp, tp)| {
            eval_cheat_value(&dists, sp.0, sp.1, tp.0, tp.1).map(|saving| (saving, 1))
        })
        .sorted_by_key(|&(saving, _)| saving)
        .filter(|&(s, _)| s >= 100)
        .into_grouping_map()
        .sum();

    info!(result = result.into_values().sum::<usize>());

    Ok(())
}

fn find_cheat_positions(
    map: &[Vec<char>],
    x: usize,
    y: usize,
) -> impl IntoIterator<Item = (usize, usize)> + use<'_> {
    Itertools::cartesian_product(0..=4, 0..=4)
        .filter(|&(dx, dy)| (dx as isize - 2).abs() + (dy as isize - 2).abs() == 2)
        .filter_map(move |(dx, dy)| Option::zip((x + dx).checked_sub(2), (y + dy).checked_sub(2)))
        .filter(|&(nx, ny)| ny < map.len() && nx < map[ny].len() && map[ny][nx] == '.')
        .filter(move |&(nx, ny)| {
            if x.abs_diff(nx) == 1 && y.abs_diff(ny) == 1 {
                map[y][nx] == '#' && map[ny][x] == '#'
            } else {
                assert!(x.abs_diff(nx) == 2 || y.abs_diff(ny) == 2);
                let mx = (x + nx) / 2;
                let my = (y + ny) / 2;
                map[my][mx] == '#'
            }
        }).collect::<Vec<_>>()
}

fn eval_cheat_value(
    dists: &HashMap<(usize, usize), usize>,
    sx: usize,
    sy: usize,
    tx: usize,
    ty: usize,
) -> Option<usize> {
    dists[&(tx, ty)].checked_sub(dists[&(sx, sy)] + 2)
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
