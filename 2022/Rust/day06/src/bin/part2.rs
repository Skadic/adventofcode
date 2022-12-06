fn main() {
    println!("Part 2: {}", include_bytes!("../../input.txt").windows(14)
        .position(|slice| {
            let mut v = slice.to_owned();
            v.sort_unstable();
            let before = v.len();
            v.dedup();
            v.len() == before
        }).unwrap() + 14);
}
