use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let mut input = parse_input(INPUT);

    let mut start = 0;
    let mut end = input.len() - 1;

    while start < end {
        while input[start].is_full() {
            start += 1;
        }
        while input[end].is_empty() {
            end -= 1;
        }
        let free_size = input[start].padding();
        let mut sliced = input[end].slice_off(free_size);
        if input[start].id == sliced.id {
            input[start].grow(sliced.len());
        } else {
            // Move Padding to sliced section
            sliced.padding = input[start].padding;
            input[start].padding = 0;
            // Remove the padding that will be taken up by the sliced section
            sliced.reduce_padding(sliced.len());
            input.insert(start + 1, sliced);
            end += 1;
        }
    }

    let result: usize = input
        .into_iter()
        .flat_map(|sec| std::iter::repeat(sec.id).take(sec.len()))
        .enumerate()
        .map(|(i, v)| i * v)
        .sum();

    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
