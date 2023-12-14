fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day14::part2::process().unwrap();
}

