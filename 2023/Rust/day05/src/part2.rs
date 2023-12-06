use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (seeds, input) = parse_input(INPUT);
    let seeds = seeds
        .chunks_exact(2)
        .map(|s| s[0]..=s[0] + s[1] - 1)
        .collect::<Vec<_>>();

    let mut rng = seeds;
    for i in 0..7 {
        rng = rng
            .into_iter()
            .flat_map(|v| input.map_range(i, v))
            .collect::<Vec<_>>();
    }
    info!(result = rng.into_iter().map(|v| *v.start()).min().unwrap());
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().init();
    process()
}
