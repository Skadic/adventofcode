fn main() {
    divan::main();
}


#[divan::bench]
fn part2_slow() {
    day06::part2::process_slow().unwrap();
}

#[divan::bench]
fn part2_fast() {
    day06::part2::process_fast().unwrap();
}

