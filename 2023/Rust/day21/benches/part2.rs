fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day21::part2::process().unwrap();
}

