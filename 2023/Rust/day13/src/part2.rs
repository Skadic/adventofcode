use tracing::info;

use crate::{parse_input, INPUT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirrorDirection {
    Horizontal,
    Vertical,
}

#[tracing::instrument(name = "part2")]
pub fn process() -> miette::Result<()> {
    let maps = parse_input(INPUT);

    let mut old_reflections = vec![];

    for map in &maps {
        if let Some(cols) = find_vert(map) {
            old_reflections.push((MirrorDirection::Vertical, cols));
        }

        if let Some(rows) = find_horiz(map) {
            old_reflections.push((MirrorDirection::Horizontal, rows));
        }
    }

    let new_reflections = old_reflections
        .into_iter()
        .enumerate()
        .map(|(i, v)| find_vert_smudged(&maps[i], v).unwrap())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (dir, lines) in new_reflections {
        sum += match dir {
            MirrorDirection::Horizontal => (lines + 1) * 100,
            MirrorDirection::Vertical => lines + 1,
        }
    }

    info!(result = sum);

    // Edit here
    Ok(())
}

fn find_vert_smudged(
    map: &[Vec<char>],
    old: (MirrorDirection, usize),
) -> Option<(MirrorDirection, usize)> {
    let mut mirror_col = None;

    // Try to find vertical
    'outer: for i in 0..map[0].len() - 1 {
        let mut cols = 0;
        let mut smudge_used = false;
        loop {
            if i < cols || i + 1 + cols >= map[0].len() {
                break;
            }

            let differences = (0..map.len())
                .map(|j| map[j][i - cols])
                .zip((0..map.len()).map(|j| map[j][i + 1 + cols]))
                .filter(|(l, r)| l != r)
                .count();
            if differences == 1 && !smudge_used {
                smudge_used = false;
            } else if differences > 0 {
                break;
            }
            cols += 1;
        }
        if i + 1 == cols || i + 1 + cols == map[0].len() {
            if old.0 != MirrorDirection::Vertical || old.1 != i {
                mirror_col = Some(i);
                break 'outer;
            }
        }
    }

    match mirror_col {
        Some(c) if (MirrorDirection::Vertical, c) != old => {
            return Some((MirrorDirection::Vertical, c));
        }
        _ => {}
    }

    let mut mirror_row = None;

    // Try to find horizontal
    'outer: for i in 0..map.len() - 1 {
        let mut rows = 0;
        let mut smudge_used = false;
        loop {
            if i < rows || i + 1 + rows >= map.len() {
                break;
            }

            let differences = (0..map[0].len())
                .map(|j| map[i - rows][j])
                .zip((0..map[0].len()).map(|j| map[i + 1 + rows][j]))
                .filter(|(l, r)| l != r)
                .count();
            if differences == 1 && !smudge_used {
                smudge_used = false;
            } else if differences > 0 {
                break;
            }
            rows += 1;
        }
        if i + 1 == rows || i + 1 + rows == map.len() {
            if old.0 != MirrorDirection::Horizontal || old.1 != i {
                mirror_row = Some(i);
                break 'outer;
            }
        }
    }

    match mirror_row {
        Some(c) if (MirrorDirection::Horizontal, c) != old => {
            return Some((MirrorDirection::Horizontal, c));
        }
        _ => panic!("aua"),
    }
}

fn find_horiz(map: &[Vec<char>]) -> Option<usize> {
    let mut mirror_row = None;
    for i in 0..map.len() - 1 {
        let mut rows = 0;
        loop {
            if i < rows || i + 1 + rows >= map.len() || map[i - rows] != map[i + 1 + rows] {
                break;
            }
            rows += 1;
        }
        if i + 1 == rows || i + 1 + rows == map.len() {
            mirror_row = Some(i);
        }
    }

    mirror_row
}

fn find_vert(map: &[Vec<char>]) -> Option<usize> {
    let mut mirror_col = None;
    for i in 0..map[0].len() - 1 {
        let mut cols = 0;
        loop {
            if i < cols
                || i + 1 + cols >= map[0].len()
                || (0..map.len())
                    .map(|j| map[j][i - cols])
                    .zip((0..map.len()).map(|j| map[j][i + 1 + cols]))
                    .any(|(l, r)| l != r)
            {
                break;
            }
            cols += 1;
        }
        if i + 1 == cols || i + 1 + cols == map[0].len() {
            mirror_col = Some(i);
        }
    }

    mirror_col
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
