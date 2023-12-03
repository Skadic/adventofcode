use day03::{get_num, num_is_adjacent, parse_input, Entry};

fn main() {
    let input = std::fs::read_to_string("../../inputs/day03/input.txt").unwrap();
    let input = parse_input(&input);

    println!(
        "{:?},",
        find_numbers(input.as_slice()).into_iter().sum::<usize>()
    )
}

fn find_numbers(input: &[Vec<Entry>]) -> Vec<usize> {
    let mut nums = vec![];

    for (row, line) in input.iter().enumerate() {
        for (col, &entry) in line.iter().enumerate() {
            let num_starts = entry.is_num()
                && line
                    .get(col.wrapping_sub(1))
                    .map(|&v| !v.is_num())
                    .unwrap_or(true);
            if !num_starts || !num_is_adjacent(input, row, col) {
                continue;
            }

            nums.push(get_num(input, row, col))
        }
    }

    nums
}
