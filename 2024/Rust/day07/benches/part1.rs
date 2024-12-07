fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day07::part1::process().unwrap();
}
