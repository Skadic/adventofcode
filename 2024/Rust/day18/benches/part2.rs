fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day18::part2::process().unwrap();
}

