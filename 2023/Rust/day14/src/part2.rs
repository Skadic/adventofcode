use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let mut map = parse_input(INPUT);

    let mut prev_cycle_map = vec![map.clone()];
    let mut cycle_start = 0;
    let mut cycle_length = 0;
    for i in 1..=1_000_000_000 {
        north(&mut map);
        west(&mut map);
        south(&mut map);
        east(&mut map);
        if let Some(eq_pos) = prev_cycle_map.iter().position(|v| v == &map) {
            cycle_start = eq_pos;
            cycle_length = i - eq_pos;
            break;
        }
        prev_cycle_map.push(map.to_owned());
    }

    let rem = (1_000_000_000 - cycle_start) % cycle_length;
    info!(result = calc_load(&prev_cycle_map[cycle_start + rem]));

    // Edit here
    Ok(())
}

fn calc_load(map: &[Vec<char>]) -> usize {
    map.iter()
        .enumerate()
        .map(|(row, line)| (map.len() - row) * line.iter().copied().filter(|&c| c == 'O').count())
        .sum()
}

pub fn north(map: &mut [Vec<char>]) {
    for col in 0..map[0].len() {
        let mut current_round: usize = 0;
        let mut segment_start = 0;
        for row in 0..map.len() {
            match map[row][col] {
                'O' => current_round += 1,
                '#' => {
                    if current_round > 0 {
                        for i in segment_start..row {
                            map[i][col] = if current_round > 0 { 'O' } else { '.' };
                            current_round = current_round.saturating_sub(1);
                        }
                    }
                    segment_start = row + 1;
                    current_round = 0;
                }
                _ => {}
            }
        }
        if current_round > 0 {
            for i in segment_start..map.len() {
                map[i][col] = if current_round > 0 { 'O' } else { '.' };
                current_round = current_round.saturating_sub(1);
            }
        }
    }
}

pub fn south(map: &mut [Vec<char>]) {
    for col in 0..map[0].len() {
        let mut current_round: usize = 0;
        let mut segment_start = 0;
        for row in 0..map.len() {
            match map[row][col] {
                'O' => current_round += 1,
                '#' => {
                    if current_round > 0 {
                        for i in (segment_start..row).rev() {
                            map[i][col] = if current_round > 0 { 'O' } else { '.' };
                            current_round = current_round.saturating_sub(1);
                        }
                    }
                    segment_start = row + 1;
                    current_round = 0;
                }
                _ => {}
            }
        }
        if current_round > 0 {
            for i in (segment_start..map.len()).rev() {
                map[i][col] = if current_round > 0 { 'O' } else { '.' };
                current_round = current_round.saturating_sub(1);
            }
        }
    }
}

pub fn west(map: &mut [Vec<char>]) {
    for row in 0..map.len() {
        let mut current_round: usize = 0;
        let mut segment_start = 0;
        for col in 0..map[0].len() {
            match map[row][col] {
                'O' => current_round += 1,
                '#' => {
                    if current_round > 0 {
                        for i in segment_start..col {
                            map[row][i] = if current_round > 0 { 'O' } else { '.' };
                            current_round = current_round.saturating_sub(1);
                        }
                    }
                    segment_start = col + 1;
                    current_round = 0;
                }
                _ => {}
            }
        }
        if current_round > 0 {
            for i in segment_start..map[0].len() {
                map[row][i] = if current_round > 0 { 'O' } else { '.' };
                current_round = current_round.saturating_sub(1);
            }
        }
    }
}

pub fn east(map: &mut [Vec<char>]) {
    for row in 0..map.len() {
        let mut current_round: usize = 0;
        let mut segment_start = 0;
        for col in 0..map.len() {
            match map[row][col] {
                'O' => current_round += 1,
                '#' => {
                    if current_round > 0 {
                        for i in (segment_start..col).rev() {
                            map[row][i] = if current_round > 0 { 'O' } else { '.' };
                            current_round = current_round.saturating_sub(1);
                        }
                    }
                    segment_start = col + 1;
                    current_round = 0;
                }
                _ => {}
            }
        }
        if current_round > 0 {
            for i in (segment_start..map[0].len()).rev() {
                map[row][i] = if current_round > 0 { 'O' } else { '.' };
                current_round = current_round.saturating_sub(1);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
