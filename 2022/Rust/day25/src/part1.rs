use day25::{conv_number, snafu};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let sum = input.lines().map(str::trim).map(conv_number).sum::<isize>();
    println!("part 1: {}", snafu(sum));
}

