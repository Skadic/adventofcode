fn main() {
    divan::main();
}

#[divan::bench]
fn part2() {
    day11::part2::process().unwrap();
}
