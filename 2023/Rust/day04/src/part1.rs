use day04::{parse_input, Card};

fn main() {
    let input = std::fs::read_to_string("../../inputs/day04/input.txt").unwrap();
    println!(
        "part 1: {}",
        parse_input(&input)
            .as_slice()
            .iter()
            .map(Card::score)
            .sum::<usize>()
    );
}
