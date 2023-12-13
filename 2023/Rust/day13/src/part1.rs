use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let maps = parse_input(INPUT);

    let mut sum = 0;

    for map in &maps {
        if let Some(cols) = find_vert(map) {
            sum += cols + 1;
        }

        if let Some(rows) = find_horiz(map) {
            sum += (rows + 1) * 100;
        }
    }
    info!(sum);

    // Edit here
    Ok(())
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
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
