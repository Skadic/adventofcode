fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day05::part2::process().unwrap();
}
