fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day03::part1::process().unwrap();
}
