fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day19::part2::process().unwrap();
}

