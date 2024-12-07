use std::{cell::RefCell, collections::HashSet};

use tracing::info;

use crate::{parse_input, AdvanceState, Grid, Guard, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process_slow() -> miette::Result<()> {
    let (positions, grid, start_x, start_y) = prepare();
    let result = slow(positions.iter().copied(), &grid, start_x, start_y);

    info!(result);
    Ok(())
}

#[tracing::instrument(name="part2", parent=None)]
pub fn process_fast() -> miette::Result<()> {
    let (positions, grid, start_x, start_y) = prepare();
    let result = fast(positions.iter().copied(), &grid, start_x, start_y);

    info!(result);
    Ok(())
}


fn prepare() -> (
    HashSet<(usize, usize)>,
    RefCell<Grid>,
    usize,
    usize,
) {
    let (grid, (start_x, start_y)) = parse_input(INPUT);
    let grid = RefCell::new(grid);
    let mut guard = Guard::new(&grid, start_x, start_y);

    let mut positions = HashSet::new();
    while guard.advance(None) != AdvanceState::Border {
        positions.insert(guard.pos());
    }

    (positions, grid, start_x, start_y)
}

// ~1.1sec on M3 Pro Mac Book
fn slow(
    positions: impl IntoIterator<Item = (usize, usize)>,
    grid: &RefCell<Grid>,
    start_x: usize,
    start_y: usize,
) -> usize {
    let mut result = 0;
    for (obs_x, obs_y) in positions {
        let mut positions = HashSet::new();
        let mut guard = Guard::new(grid, start_x, start_y);
        positions.insert((guard.pos(), guard.direction));
        let is_loop = loop {
            let state = guard.advance(Some((obs_x, obs_y)));
            if state == AdvanceState::Border {
                break false;
            }
            if positions.contains(&(guard.pos(), guard.direction)) {
                break true;
            }
            positions.insert((guard.pos(), guard.direction));
        };

        if is_loop {
            result += 1;
        }
    }
    result
}

// ~45ms on M3 Pro Mac Book
fn fast(
    positions: impl IntoIterator<Item = (usize, usize)>,
    grid: &RefCell<Grid>,
    start_x: usize,
    start_y: usize,
) -> usize {
    let mut result = 0;
    for (obs_x, obs_y) in positions {
        grid.borrow_mut().set(obs_x, obs_y);
        let mut positions = HashSet::new();
        let mut guard = Guard::new(grid, start_x, start_y);
        let is_loop = loop {
            if !guard.quick_walk() {
                break false;
            }
            let pos = guard.pos();
            if positions.contains(&(pos, guard.direction)) {
                break true;
            } else {
                positions.insert((pos, guard.direction));
            }
        };

        if is_loop {
            result += 1;
        }
        grid.borrow_mut().unset(obs_x, obs_y);
    }
    result
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process_fast()?;
    process_slow()?;
    Ok(())
}
