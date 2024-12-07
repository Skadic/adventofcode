use std::{cell::RefCell, collections::HashSet};

use tracing::{error, info};

use crate::{parse_input, AdvanceState, Guard, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let (grid, (start_x, start_y)) = parse_input(INPUT);
    let grid = RefCell::new(grid);
    let mut guard = Guard::new(&grid, start_x, start_y);

    let mut positions = HashSet::new();
    positions.insert(guard.pos());
    while guard.advance(None) != AdvanceState::Border {
        positions.insert(guard.pos());
    }

    
    info!(result = positions.len());
    error!(error = %"hi".parse::<usize>().unwrap_err(), "error occurred");
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
