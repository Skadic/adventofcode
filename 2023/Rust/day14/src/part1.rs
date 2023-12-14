use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let v = parse_input(INPUT);

    info!(result = load(&v));

    Ok(())
}

pub fn load(map: &[Vec<char>]) -> usize {
    let mut load = 0;

    for col in 0..map[0].len() {
        let mut current_round = 0;
        let mut segment_start = 0;
        for row in 0..map.len() {
            match map[row][col] {
                'O' => current_round += 1,
                '#' => {
                    if current_round > 0 {
                        for i in segment_start..segment_start + current_round {
                            load += map.len() - i;
                        }
                    }
                    segment_start = row + 1;
                    current_round = 0;
                }
                _ => {}
            }
        }
        if current_round > 0 {
            for i in segment_start..segment_start + current_round {
                load += map.len() - i;
            }
        }
    }

    load
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
