fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day09::part1::process().unwrap();
}
