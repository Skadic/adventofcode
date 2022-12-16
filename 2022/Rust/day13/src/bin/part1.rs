use day13::process_input;

fn main() {
    let input = include_str!("../../input.txt");
    let packets = process_input(input);

    println!(
        "{:?}",
        packets
            .into_iter()
            .enumerate()
            .filter_map(|(i, (l, r))| if l < r { Some(i + 1) } else { None })
            .sum::<usize>()
    )
}
