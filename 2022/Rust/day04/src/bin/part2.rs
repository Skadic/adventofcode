use day04::{process_input, overlaps};

fn main() {
    let input = include_str!("../../input.txt");
    let processed = process_input(input);

    println!(
        "Part 2: {}",
        processed
            .into_iter()
            .filter(|&(l, r)| overlaps(l, r))
            .count()
    );
}
