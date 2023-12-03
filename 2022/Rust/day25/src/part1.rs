use day25::{de_snafu, snafu};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let sum = input.lines().map(str::trim).map(de_snafu).sum::<isize>();
    println!("part 1: {}", snafu(sum));
}

