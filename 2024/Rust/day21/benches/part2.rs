fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day21::part2::process().unwrap();
}

