fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day11::part2::process().unwrap();
}

