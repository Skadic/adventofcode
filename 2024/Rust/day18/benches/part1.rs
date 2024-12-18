fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day18::part1::process().unwrap();
}
