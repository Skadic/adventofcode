fn main() {
    divan::main();
}


#[divan::bench(sample_count = 10000)]
fn part1() {
    day08::part2::process().unwrap();
}

