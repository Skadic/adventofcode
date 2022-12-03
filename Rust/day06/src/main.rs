fn main() {
    let input = std::fs::read_to_string("res/input.txt").unwrap();

    let fish = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i8>>();

    println!("Part 1: {}", calculate(&fish, 80));
    println!("Part 2: {}", calculate(&fish, 256));
}

fn calculate(fish: &[i8], days: usize) -> usize {
    let mut fish_count = vec![0usize; 9]; // Fish count at 0 to 8 days

    for i in 0..7i8 {
        fish_count[i as usize] = fish.iter().filter(|&&n| n == i).count();
    }

    let mut base_line = 0usize; // What index is "day zero" in the vec
    for _ in 0..days {
        let entering = fish_count[7];
        fish_count[7] = fish_count[8];
        fish_count[8] = fish_count[base_line];
        fish_count[base_line] += entering;
        base_line = (base_line + 1) % 7;
    }

    fish_count.into_iter().sum()
}
