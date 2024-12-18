use tracing::info;

use crate::{bfs, parse_input, INPUT};

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (bytes, mut map) = parse_input(INPUT);

    for (x, y) in bytes.into_iter() {
        map[y][x] = '#';
        let dists = bfs(&map, 0, 0);
        if !dists.contains_key(&(map.len() - 1, map.len() - 1)) {
            info!(result = ?(x, y));
            return Ok(());
        }
    }
    //info!(?dists);

    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
