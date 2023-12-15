fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day15::part2::process().unwrap();
}

