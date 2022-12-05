use day04::{includes_second, process_input};

fn main() {
    let input = include_str!("../../input.txt");
    let processed = process_input(input);

    println!(
        "Part 1: {}",
        processed
            .into_iter()
            .filter(|&(l, r)| includes_second(l, r) || includes_second(r, l))
            .count()
    );
}
