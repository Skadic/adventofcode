fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day10::part1::process().unwrap();
}
