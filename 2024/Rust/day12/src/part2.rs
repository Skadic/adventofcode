use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

use tracing::info;

use crate::{parse_input, Plot, Regions, INPUT};

// TOO LOW 454494

#[tracing::instrument(name = "part2", parent=None)]
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
                        convert_all(&input, ids.as_mut_slice(), x, y - 1, left_id);
                        let u_plot = regions.plots.remove(&(c, up_id)).unwrap();
                        regions
                            .plots
                            .entry((c, left_id))
                            .and_modify(|plot| {
                                plot.area += u_plot.area;
                                plot.perim += u_plot.perim;
                                plot.sides += u_plot.sides;
                            })
                            .or_insert(u_plot);
                        left_id
                    }
                    Ordering::Greater => {
                        convert_all(&input, ids.as_mut_slice(), x - 1, y, up_id);
                        let l_plot = regions.plots.remove(&(c, left_id)).unwrap();
                        regions
                            .plots
                            .entry((c, up_id))
                            .and_modify(|plot| {
                                plot.area += l_plot.area;
                                plot.perim += l_plot.perim;
                                plot.sides += l_plot.sides;
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
            plot.sides += PlotMod::determine(&input, c, x, y).side_increase();
        }
    }

    let result: usize = regions
        .plots
        .into_values()
        .map(|Plot { area, sides, .. }| area * sides as usize)
        .sum();
    info!(result);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlotMod {
    NewPlot,
    Extend,
    Angle,
    Break,
    Join,
}

impl PlotMod {
    fn side_increase(self) -> isize {
        match self {
            Self::NewPlot => 4,
            Self::Extend => 0,
            Self::Angle => 2,
            Self::Break => 4,
            Self::Join => -2,
        }
    }

    fn determine(chars: &[Vec<char>], c: char, x: usize, y: usize) -> Self {
        let tl = get_safe_and_match(chars, x.wrapping_sub(1), y.wrapping_sub(1), c);
        let t = get_safe_and_match(chars, x, y.wrapping_sub(1), c);
        let tr = get_safe_and_match(chars, x + 1, y.wrapping_sub(1), c);
        let l = get_safe_and_match(chars, x.wrapping_sub(1), y, c);

        // Left is something else
        if !l {
            if !t {
                return Self::NewPlot;
            }

            if !tl && t && tr {
                return Self::Angle;
            }

            if tl && t && !tr {
                return Self::Angle;
            }

            if !tl && t && !tr {
                return Self::Extend;
            }

            if tl && t && tr {
                return Self::Break;
            }
        } else {
            // Left is true now
            if t && tr {
                return Self::Extend;
            }

            if t && !tr {
                return Self::Join;
            }

            if tl && !t {
                return Self::Angle;
            }

            if !tl && !t {
                return Self::Extend;
            }
        }
        panic!("aua {l:?}, {tl:?}, {t:?}, {tr:?}")
    }
}

fn convert_all(chars: &[Vec<char>], ids: &mut [Vec<usize>], x: usize, y: usize, to: usize) {
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

fn get_safe_and_match(v: &[Vec<char>], x: usize, y: usize, c: char) -> bool {
    v.get(y)
        .and_then(|row| row.get(x))
        .map(|&ch| ch == c)
        .unwrap_or(false)
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
