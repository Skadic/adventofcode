use tracing::info;

use crate::{parse_input, INPUT};

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let mut input = parse_input(INPUT);

    let mut end = input.len() - 1;

    while end > 0 {
        let mut start = 0;
        while input[start].padding() < input[end].len() && start < end {
            start += 1;
        }
        if start == end {
            end -= 1;
            continue;
        }

        // Move Padding to sliced section
        input[end - 1].padding += input[end].len() + input[end].padding();
        input[end].padding = input[start].padding;
        input[start].padding = 0;
        // Remove the padding that will be taken up by the sliced section
        let len = input[end].len();
        input[end].reduce_padding(len);
        input.insert(start + 1, input[end]);
        // Since we inserted a value before, input[end] moved one position along
        input.remove(end + 1);
    }

    let result: usize = input
        .into_iter()
        .flat_map(|sec| {
            std::iter::repeat(Some(sec.id))
                .take(sec.len())
                .chain(std::iter::repeat(None).take(sec.padding()))
        })
        .enumerate()
        .filter_map(|(i, v)| v.map(|v| i * v))
        .sum();

    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
