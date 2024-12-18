use tracing::info;

use crate::{bfs, parse_input, INPUT};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let (bytes, mut map) = parse_input(INPUT);

    for (x,y) in bytes.into_iter().take(1024) {
        map[y][x] = '#';
    }    
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!()
    }

    let dists = bfs(&map, 0, 0);
    //info!(?dists);

    info!(result = dists[&(map.len() - 1, map.len() - 1)]);

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
