use std::{cmp::Ordering, collections::VecDeque};

use aho_corasick::AhoCorasick;
use itertools::Itertools;
use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let (towels, combos) = parse_input(INPUT);
    let aho = AhoCorasick::new(&towels).unwrap();

    let combo_str = combos.join("#");
    let mut matches = aho
        .find_overlapping_iter(&combo_str)
        .map(|m| (m.span().start, m.span().end))
        .into_group_map();
    let matches = (0..combo_str.len())
        .map(|i| matches.get_mut(&i).map(std::mem::take).unwrap_or(vec![]))
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut rest = combo_str.as_str();
    let mut target_start = 0;
    let mut remaining = VecDeque::with_capacity(100);
    while !rest.is_empty() {
        let hash_pos = rest.find("#").unwrap_or(rest.len());
        remaining.clear();
        remaining.push_back(target_start);

        while let Some(start) = remaining.pop_back() {
            match start.cmp(&(target_start + hash_pos)) {
                Ordering::Equal => {
                    result += 1;
                    break;
                }
                Ordering::Less => {
                    remaining.extend(&matches[start]);
                }
                Ordering::Greater => {}
            }
        }

        target_start += hash_pos + 1;
        rest = &rest[(hash_pos + 1).min(rest.len())..]
    }

    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
