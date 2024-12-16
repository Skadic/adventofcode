fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day16::part2::process().unwrap();
}

