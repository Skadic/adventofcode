fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day20::part1::process().unwrap();
}
