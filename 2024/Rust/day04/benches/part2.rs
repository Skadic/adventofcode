fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day04::part2::process().unwrap();
}

