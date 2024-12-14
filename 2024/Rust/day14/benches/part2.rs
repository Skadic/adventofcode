fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day14::part2::process().unwrap();
}

