fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day17::part2::process().unwrap();
}

