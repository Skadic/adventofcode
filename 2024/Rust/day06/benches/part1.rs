fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day06::part1::process().unwrap();
}
