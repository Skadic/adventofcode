use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument]
/// Greatest Common Divisor
fn gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if a == 0 || b == 0 {
            return a.max(b);
        }
        match a.cmp(&b) {
            Ordering::Greater => a %= b,
            Ordering::Less => b %= a,
            Ordering::Equal => return a,
        }
    }
}

/// Least Common Multiple
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (rl, current, dirs) = parse_input(INPUT);

    info!(
        result = current
            .into_iter()
            .map(|cur| find_cycles(&rl, &dirs, cur))
            .fold(1, |acc, (first_z, cycles)| lcm(first_z % cycles, acc))
    );
    Ok(())
}

/// Returns the position of the first end node, the cycle size after which the same sequence
/// repeats
fn find_cycles<'a>(
    rl: &[usize],
    dirs: &HashMap<&'a str, [&'a str; 2]>,
    mut current: &'a str,
) -> (usize, usize) {
    let mut iter = rl.iter().enumerate();

    let mut steps = vec![current];
    let mut set = HashSet::new();

    let cycle_size;
    loop {
        if let Some((i, &next)) = iter.next() {
            if set.contains(&(current, i)) {
                let i = steps.iter().position(|&s| s == current).unwrap();
                cycle_size = steps.len() - i;
                break;
            }
            set.insert((current, i));
            current = dirs[current][next];
            steps.push(current);
        } else {
            iter = rl.iter().enumerate();
        }
    }

    (
        steps.iter().position(|v| v.ends_with('Z')).unwrap(),
        cycle_size,
    )
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
