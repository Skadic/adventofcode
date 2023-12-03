use day03::{adjacent_to_two, gear_ratio, parse_input, Entry};

fn main() {
    let input = std::fs::read_to_string("../../inputs/day03/input.txt").unwrap();
    let input = parse_input(&input);

    println!(
        "part 2: {:?},",
        find_gears(input.as_slice()).into_iter().sum::<usize>()
    )
}

fn find_gears(input: &[Vec<Entry>]) -> Vec<usize> {
    let mut nums = vec![];

    for (row, line) in input.iter().enumerate() {
        for (col, &entry) in line.iter().enumerate() {
            if matches!(entry, Entry::Gear) && adjacent_to_two(input, row, col) {
                nums.push(gear_ratio(input, row, col))
            }
        }
    }

    nums
}
