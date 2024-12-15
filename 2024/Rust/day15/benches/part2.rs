fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day15::part2::process().unwrap();
}

