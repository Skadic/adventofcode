fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day13::part2::process().unwrap();
}

