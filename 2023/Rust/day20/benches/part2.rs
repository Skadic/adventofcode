fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day20::part2::process().unwrap();
}

