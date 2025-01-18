fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day22::part1::process().unwrap();
}
