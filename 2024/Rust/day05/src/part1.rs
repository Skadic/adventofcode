use std::collections::{HashMap, HashSet};

use tracing::info;

use crate::{parse_input, Orders, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let Orders { pages, rules } = parse_input(INPUT);

    let mut rule_map = HashMap::new();
    for (pred, succ) in rules {
        rule_map.entry(succ).or_insert_with(|| vec![]).push(pred);
    }

    let result: usize = pages
        .into_iter()
        .filter(|pages| {
            let mut forbidden = HashSet::<usize>::new();
            for page in pages {
                if forbidden.contains(page) {
                    return false;
                }
                if let Some(pred) = rule_map.get(page) {
                    forbidden.extend(pred);
                }
            }
            true
        })
        .map(|pages| pages[pages.len() / 2])
        .sum();

    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
