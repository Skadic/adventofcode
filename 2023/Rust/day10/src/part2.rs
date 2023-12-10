use tracing::info;

use crate::{bfs, parse_input, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let mut map = parse_input(INPUT);
    let bfs_map = bfs(&map);
    for row in 0..map.height() {
        for col in 0..map.width() {
            if bfs_map[row][col] < 0 {
                map.grid[row][col] = ' ';
            }
        }
    }

    let mut count = 0;
    for row in 0..map.height() {
        let mut num_borders = 0;
        let mut inside = false;
        let mut border_start = None;
        for col in 0..map.width() {
            let current = map.get(col, row);
            if bfs_map[row][col] >= 0 {
                if num_borders > 0 && current == '-' {
                    continue;
                }
                if "LF".contains(current) {
                    border_start = Some(current);
                    num_borders += 1;
                    continue;
                }
                if let Some(bs) = border_start {
                    match bs {
                        'L' if current == 'J' => num_borders -= 1,
                        'F' if current == '7' => num_borders -= 1,
                        _ => {}
                    }
                    border_start = None;
                    continue;
                }
                num_borders += 1;
                continue;
            }
            if num_borders > 0 {
                inside ^= num_borders % 2 == 1;
                num_borders = 0;
            }
            map.grid[row][col] = if inside { 'I' } else { 'O' };
            if inside {
                count += 1;
            }
        }
    }
    info!(result = count);
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
