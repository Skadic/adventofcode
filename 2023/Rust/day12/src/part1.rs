use tracing::info;

use crate::{parse_input, SpringState, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let v = parse_input(INPUT);

    let mut cnt = 0;
    for (states, ranges) in v.into_iter() {
        let post_len = ranges.iter().sum::<usize>() + ranges.len();
        let first_range_len = ranges[0];
        let line_res = place(&states, &ranges, post_len - first_range_len - 1);
        cnt += line_res;
    }

    info!(result = cnt);
    // Edit here
    Ok(())
}

fn place(states: &[SpringState], ranges: &[usize], post_len: usize) -> usize {
    if ranges.is_empty() {
        if states.iter().all(|&s| s != SpringState::Damaged) {
            return 1;
        } else {
            return 0;
        }
    }
    let assigned_positions = assignable_pos(states, ranges[0], 0, post_len, None, None);
    if assigned_positions.is_empty() {
        return 0;
    }

    let range_len = ranges[0];
    let mut cnt = 0;
    for pos in assigned_positions {
        //info!(post_len, range_len, res = post_len - range_len - 1);
        let c = place(
            &states[(pos + range_len + 1).min(states.len())..],
            &ranges[1..],
            post_len
                .saturating_sub(*ranges.get(1).unwrap_or(&0))
                .saturating_sub(1),
        );
        cnt += c;
    }

    cnt
}

fn nth_dmg(states: &[SpringState], n: usize) -> Option<usize> {
    states
        .iter()
        .enumerate()
        .filter(|&(_, &s)| s == SpringState::Damaged)
        .map(|(i, _)| i)
        .nth(n)
}

#[tracing::instrument(skip(states, prev_pos, prev_len))]
fn assignable_pos(
    states: &[SpringState],
    range_len: usize,
    pre_len: usize,
    post_len: usize,
    prev_pos: Option<&[usize]>,
    prev_len: Option<usize>,
) -> Vec<usize> {
    let first_prev_pos = *prev_pos.and_then(|arr| arr.first()).unwrap_or(&0);
    let first_dmg_pos = nth_dmg(states, 0).unwrap_or(states.len());
    let start_pos = pre_len.max(first_prev_pos + prev_len.map(|v| v + 1).unwrap_or(0));
    let end_pos = (states.len() - range_len - post_len + 1).min(first_dmg_pos + 1);
    let mut assignable = vec![];
    for i in start_pos..end_pos {
        let prev_not_damaged = i == 0 || states[i - 1] != SpringState::Damaged;
        let succ_not_damaged =
            i + range_len >= states.len() || states[i + range_len] != SpringState::Damaged;

        let all_placeable = states[i..i + range_len]
            .iter()
            .all(|&s| matches!(s, SpringState::Damaged | SpringState::Unknown));
        if prev_not_damaged && succ_not_damaged && all_placeable {
            assignable.push(i);
        }
    }
    assignable
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
