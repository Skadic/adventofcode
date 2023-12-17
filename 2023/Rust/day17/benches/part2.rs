fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day17::part2::process().unwrap();
}

