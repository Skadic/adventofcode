use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

use crate::{parse_input, Plot, Regions, INPUT};
use tracing::info;

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);
    let mut ids: Vec<Vec<usize>> = Vec::with_capacity(input.len());
    let mut regions: Regions = Default::default();

    for y in 0..input.len() {
        ids.push(Vec::with_capacity(input[y].len()));
        for x in 0..input[y].len() {
            let c = input[y][x];
            let mut added_perim = 4;

            let left_id =
                if let Some(&right) = input.get(y).and_then(|row| row.get(x.wrapping_sub(1))) {
                    if right == c {
                        added_perim -= 2;
                        Some(ids[y][x - 1])
                    } else {
                        None
                    }
                } else {
                    None
                };
            let up_id = if let Some(&up) = input.get(y.wrapping_sub(1)).and_then(|row| row.get(x)) {
                if up == c {
                    added_perim -= 2;
                    Some(ids[y - 1][x])
                } else {
                    None
                }
            } else {
                None
            };

            let id = if let Some((left_id, up_id)) = Option::zip(left_id, up_id) {
                match left_id.cmp(&up_id) {
                    Ordering::Equal => left_id,
                    Ordering::Less => {
                        merge_plots(&input, ids.as_mut_slice(), x, y - 1, left_id);
                        let u_plot = regions.plots.remove(&(c, up_id)).unwrap();
                        regions
                            .plots
                            .entry((c, left_id))
                            .and_modify(|plot| {
                                plot.area += u_plot.area;
                                plot.perim += u_plot.perim;
                            })
                            .or_insert(u_plot);
                        left_id
                    }
                    Ordering::Greater => {
                        merge_plots(&input, ids.as_mut_slice(), x - 1, y, up_id);
                        let l_plot = regions.plots.remove(&(c, left_id)).unwrap();
                        regions
                            .plots
                            .entry((c, up_id))
                            .and_modify(|plot| {
                                plot.area += l_plot.area;
                                plot.perim += l_plot.perim;
                            })
                            .or_insert(l_plot);
                        up_id
                    }
                }
            } else {
                left_id.or(up_id).unwrap_or_else(|| {
                    let id = regions.ids.entry(c).or_insert(0);
                    let id_to_return = *id;
                    *id += 1;
                    regions.plots.insert((c, id_to_return), Default::default());
                    id_to_return
                })
            };

            ids[y].push(id);
            let plot = regions.plots.entry((c, id)).or_insert(Plot::default());
            plot.perim += added_perim;
            plot.area += 1;
        }
    }

    let result: usize = regions
        .plots
        .into_values()
        .map(|Plot { area, perim, .. }| area * perim)
        .sum();
    info!(result);
    Ok(())
}

// yup, it's dfs again
fn merge_plots(chars: &[Vec<char>], ids: &mut [Vec<usize>], x: usize, y: usize, to: usize) {
    let from = ids[y][x];
    let c = chars[y][x];
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    let mut visited = HashSet::new();

    while let Some(current @ (current_x, current_y)) = queue.pop_back() {
        ids[current_y][current_x] = to;
        visited.insert(current);

        queue.extend(
            [
                (current_x.wrapping_sub(1), current_y),
                (current_x + 1, current_y),
                (current_x, current_y.wrapping_sub(1)),
                (current_x, current_y + 1),
            ]
            .into_iter()
            .filter_map(|(x, y)| ids.get(y).and_then(|row| row.get(x)).map(|&v| ((x, y), v)))
            .filter(|&(_, v)| v == from)
            .map(|(pos, _)| pos)
            .filter(|&(x, y)| chars[y][x] == c)
            .filter(|pos| !visited.contains(pos)),
        );
    }
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
