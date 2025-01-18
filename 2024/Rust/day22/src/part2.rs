use std::iter::zip;

use tracing::info;

use crate::{next_secret_number, parse_input, INPUT};

//#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);

    let mut scores = vec![0; 150_000];
    let (bananas, changes) = calculate_bananas(&input);

    for (bananas, changes) in zip(bananas, changes) {
        let mut local_scores = vec![None; 150_000];

        for (window, score) in zip(changes.windows(4), bananas.into_iter().skip(3)) {
            let window: [i8; 4] = window.try_into().unwrap();
            let idx = make_index(window);
            if local_scores[idx].is_none() {
                local_scores[idx] = Some(score);
            }
        }

        for (idx, score) in local_scores.into_iter().enumerate() {
            if let Some(score) = score {
                scores[idx] += score as usize;
            }
        }
    }

    let result = scores.into_iter().max().unwrap();

    info!(?result);

    Ok(())
}

fn calculate_bananas(input: &[usize]) -> (Vec<Vec<i8>>, Vec<Vec<i8>>) {
    let mut bananas = Vec::new();
    let mut changes = Vec::new();

    for buyer_secret_number in input.iter().copied() {
        let mut buyer_bananas = Vec::new();
        let mut buyer_changes = Vec::new();
        let mut current = buyer_secret_number;
        for _ in 0..2000 {
            let prev = current;
            current = next_secret_number(current);
            buyer_bananas.push((current % 10) as i8);
            buyer_changes.push((current % 10) as i8 - (prev % 10) as i8);

            // println!("{}: {} - {:?}", current, buyer_bananas.last().unwrap(), &buyer_changes[buyer_changes.len().saturating_sub(4)..])
        }
        bananas.push(buyer_bananas);
        changes.push(buyer_changes);
    }

    (bananas, changes)
}

fn make_index(window: [i8; 4]) -> usize {
    window
        .iter()
        .map(|r| *r + 9)
        .fold(0, |acc, i| acc * 18 + i as usize)
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
