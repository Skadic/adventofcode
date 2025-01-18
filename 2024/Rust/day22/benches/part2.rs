fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day22::part2::process().unwrap();
}

