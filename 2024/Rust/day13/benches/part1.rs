fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day13::part1::process().unwrap();
}
