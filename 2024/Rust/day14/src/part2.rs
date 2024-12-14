use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use itertools::Itertools;

use crate::{lcm, parse_input, INPUT};

struct Bots(HashSet<(usize, usize)>, usize, usize);

impl Bots {
    fn could_work(&self) -> bool {
        for y in 0..self.2 {
            for mut chunk in &(0..self.1).chunks(8) {
                if chunk.all(|x| self.0.contains(&(x, y))) {
                    return true;
                }
            }
        }
        false
    }
}

impl Display for Bots {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.2 {
            let mut v = String::with_capacity(self.1);
            for x in 0..self.1 {
                if self.0.contains(&(x, y)) {
                    v.push('#');
                } else {
                    v.push('.');
                }
            }
            writeln!(f, "{v}")?;
        }
        Ok(())
    }
}

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (input_str, w, h) = INPUT;
    let mut input = parse_input(input_str);

    let cycle_length = lcm(w, h);
    for i in 1..=cycle_length {
        for bot in &mut input {
            bot.move_once(w, h);
        }
        let bots = Bots(
            input
                .iter()
                .map(|&bot| (bot.x as usize, bot.y as usize))
                .collect(),
            w,
            h,
        );
        if bots.could_work() {
            println!("{bots}");
            println!("This was cycle {i}/{cycle_length}");
            let mut v = String::new();
            std::io::stdin().read_line(&mut v).unwrap();
        }
    }

    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
