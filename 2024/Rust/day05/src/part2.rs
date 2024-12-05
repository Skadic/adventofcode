use std::collections::{HashMap, HashSet};

use tracing::info;

use crate::{parse_input, Orders, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let Orders { pages, rules } = parse_input(INPUT);

    let mut rule_map = HashMap::new();
    for (pred, succ) in rules {
        rule_map.entry(succ).or_insert_with(|| vec![]).push(pred);
    }

    let mut incorrect_orders = pages
        .into_iter()
        .filter(|pages| {
            let mut forbidden = HashSet::<usize>::new();
            for page in pages {
                if forbidden.contains(page) {
                    return true;
                }
                if let Some(pred) = rule_map.get(page) {
                    forbidden.extend(pred);
                }
            }
            false
        })
        .collect::<Vec<_>>();

    let mut order_idx = 0;
    'outer: while order_idx < incorrect_orders.len() {
        let pages = &mut incorrect_orders[order_idx];
        let mut forbidden = HashMap::<usize, usize>::new();
        for i in 0..pages.len() {
            if let Some(&pos) = forbidden.get(&pages[i]) {
                pages.swap(i, pos);
                continue 'outer;
            }
            if let Some(preds) = rule_map.get(&pages[i]) {
                forbidden.extend(preds.iter().map(|&v| (v, i)));
            }
        }
        order_idx += 1;
    }

    let result: usize = incorrect_orders
        .into_iter()
        .map(|pages| pages[pages.len() / 2])
        .sum();

    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
