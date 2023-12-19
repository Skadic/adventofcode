fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day19::part2::process().unwrap();
}

