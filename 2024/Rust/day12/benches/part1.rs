fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day12::part1::process().unwrap();
}
