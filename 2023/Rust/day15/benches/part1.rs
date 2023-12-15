fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day15::part1::process().unwrap();
}
