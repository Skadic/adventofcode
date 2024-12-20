fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day20::part2::process().unwrap();
}

