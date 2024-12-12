fn main() {
    divan::main();
}

#[divan::bench]
fn part2() {
    day12::part2::process().unwrap();
}
