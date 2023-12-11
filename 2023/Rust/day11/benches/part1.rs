fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day11::part1::process().unwrap();
}
