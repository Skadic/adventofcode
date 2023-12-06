use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let (seeds, input) = parse_input(INPUT);

    let mut rng = seeds.into_iter().map(|v| v..=v).collect::<Vec<_>>();
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
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
