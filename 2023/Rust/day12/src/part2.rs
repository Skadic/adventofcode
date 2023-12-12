use tracing::info;

use crate::{parse_input, SpringState, INPUT};

#[tracing::instrument(name = "part2")]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT)
        .into_iter()
        .map(|(mut states, ranges)| {
            states.push(SpringState::Unknown);
            let n_states = states.len();
            let n_ranges = ranges.len();
            (
                states
                    .into_iter()
                    .cycle()
                    .take(n_states * 5 - 1)
                    .collect::<Vec<_>>(),
                ranges
                    .into_iter()
                    .cycle()
                    .take(n_ranges * 5)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    info!(
        result = input
            .into_iter()
            .map(|(states, ranges)| place3(&states, &ranges))
            .sum::<usize>()
    );

    // Edit here
    Ok(())
}

// Too low

fn can_place(end_i: usize, range_len: usize, states: &[SpringState]) -> bool {
    if end_i < range_len - 1 {
        return false;
    }

    if end_i >= range_len && states[end_i - range_len] == SpringState::Damaged {
        return false;
    }

    if states
        .get(end_i + 1)
        .map(|&s| s == SpringState::Damaged)
        .unwrap_or(false)
    {
        return false;
    }

    for &state in &states[end_i + 1 - range_len..=end_i] {
        if state == SpringState::Operational {
            return false;
        }
    }
    true
}

fn check_overlaps(
    range: usize,
    mut pos: usize,
    table: &mut [Vec<usize>],
    states: &[SpringState],
) -> bool {
    while pos > 0 {
        if table[range - 1][pos] > 0 {
            return true;
        }

        if states[pos - 1] == SpringState::Damaged {
            return false;
        }
        pos -= 1;
    }
    range == 1
}

fn get_ways(
    range: usize,
    pos: usize,
    range_len: usize,
    table: &[Vec<usize>],
    states: &[SpringState],
) -> usize {
    let mut c = if range_len == pos {
        table[range - 1][0]
    } else {
        0
    };
    for p in (0..pos - range_len).rev() {
        c += table[range - 1][p];
        if p > 0 && states[p - 1] == SpringState::Damaged {
            break;
        }
    }
    c
}

fn place3(states: &[SpringState], ranges: &[usize]) -> usize {
    let mut table = vec![vec![0; states.len() + 1]; ranges.len() + 1];
    table[0][0] = 1;

    for (range_idx, &range_len) in ranges.iter().enumerate() {
        let range = range_idx + 1;
        for idx in 0..states.len() {
            let pos = idx + 1;
            if can_place(pos - 1, range_len, states)
                && check_overlaps(
                    range,
                    pos.saturating_sub(range_len).saturating_sub(1),
                    &mut table,
                    states,
                )
            {
                let ways = get_ways(range, pos, range_len, &table, states);
                table[range][pos] = ways;
            }
        }
    }

    let mut cnt = 0;
    for (i, v) in table
        .into_iter()
        .last()
        .unwrap()
        .into_iter()
        .enumerate()
        .rev()
    {
        cnt += v;
        if i > 0 && states[i - 1] == SpringState::Damaged {
            break;
        }
    }
    cnt
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
