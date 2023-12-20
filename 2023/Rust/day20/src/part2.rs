use std::cmp::Ordering;

use tracing::info;


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
    // look at input in graphviz. yes it's day 8 all over again
    // very fun /s
    let x = 0b1111_1110_1111;
    let y = 0b1110_1101_0101;
    let z = 0b1111_0100_1111;
    let w = 0b1110_1011_0001;
    info!(res = lcm(lcm(x,y), lcm(z,w)));
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
