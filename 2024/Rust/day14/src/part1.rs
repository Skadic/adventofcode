use tracing::info;

use crate::{lcm, parse_input, INPUT};

const CYCLES: usize = 100;

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let (input_str, w, h) = INPUT;

    let mut input = parse_input(input_str);
    let cycle_length = lcm(w, h);
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for bot in &mut input {
        bot.move_multiple(w, h, CYCLES % cycle_length);
        let x = bot.x - (w as isize) / 2;
        let y = bot.y - (h as isize) / 2;

        match (x.signum(), y.signum()) {
            (1, 1) => q1 += 1,
            (-1, 1) => q2 += 1,
            (-1, -1) => q3 += 1,
            (1, -1) => q4 += 1,
            _ => {}
        }
    }

    info!(q1, q2, q3, q4);

    info!(result = q1 * q2 * q3 * q4);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
