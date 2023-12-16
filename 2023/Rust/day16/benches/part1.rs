fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day16::part1::process().unwrap();
}
