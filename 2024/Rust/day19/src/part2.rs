use itertools::Itertools;
use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (mut towels, combos) = parse_input(INPUT);
    // Sort the towels by length
    // This is important for chunk_by, since it expects the input to be sorted in order to group correctly
    // Otherwise, it would just group consecutive elements with the same key
    towels.sort_unstable_by_key(|t| t.len());

    let mut result = 0;
    for combo in combos {
        // Number of combos that end at i (exclusive).
        let mut num_combos = Vec::with_capacity(combo.len() + 1);
        // There is only one combo that match 0 characters: Taking no towel at all
        num_combos.push(1);

        for i in 1..=combo.len() {
            let mut possibilities = 0usize;
            // Take all towels that do not start before the start of the string and group them by length
            let valid_towels = towels
                .iter()
                .take_while(|t| t.len() <= i)
                .chunk_by(|t| t.len());

            // Check for each group of towels, if there is one that matches at the current position
            for (len, towels) in &valid_towels {
                if towels
                    .into_iter()
                    .any(|&towel| combo[i - len..].starts_with(towel))
                {
                    possibilities += num_combos[i - len];
                    continue;
                }
            }
            num_combos.push(possibilities);
        }

        result += num_combos.last().unwrap();
    }

    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
