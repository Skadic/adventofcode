use std::{collections::HashMap, iter::zip};

use tracing::info;

use crate::{next_secret_number, parse_input, INPUT};

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);

    let mut scores = HashMap::new();
    let (bananas, changes) = calculate_bananas(&input);

    for (bananas, changes) in zip(bananas, changes) {
        let mut local_scores = HashMap::new();

        for (window, score) in zip(changes.windows(4), bananas.into_iter().skip(3)) {
            let window: [isize; 4] = window.try_into().unwrap();
            local_scores.entry(window).or_insert(score);
        }

        for (window, score) in local_scores.into_iter() {
            let entry = scores.entry(window).or_insert(0);
            *entry += score;
        }
    }


    let result = scores.into_iter().max_by_key(|(_, score)| *score).unwrap();

    info!(?result);

    Ok(())
}

fn calculate_bananas(input: &[usize]) -> (Vec<Vec<isize>>, Vec<Vec<isize>>) {
    let mut bananas = Vec::new();
    let mut changes = Vec::new();

    for buyer_secret_number in input.iter().copied() {
        let mut buyer_bananas = Vec::new();
        let mut buyer_changes = Vec::new();
        let mut current = buyer_secret_number;
        for _ in 0..2000 {
            let prev = current;
            current = next_secret_number(current);
            buyer_bananas.push(current as isize % 10);
            buyer_changes.push(current as isize % 10 - prev as isize % 10);

            // println!("{}: {} - {:?}", current, buyer_bananas.last().unwrap(), &buyer_changes[buyer_changes.len().saturating_sub(4)..])
        }
        bananas.push(buyer_bananas);
        changes.push(buyer_changes);
    }

    (bananas, changes)
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
