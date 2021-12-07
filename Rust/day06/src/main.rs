fn main() {
    let input = std::fs::read_to_string("res/input.txt").unwrap();

    let fish = input.trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i8>>();

    println!("Part 1: {}", part1(fish.clone()));
    println!("Part 2: {}", part2(fish.clone()));
}

fn part1(mut fish: Vec<i8>) -> usize {
    for _ in 0..80 {
        for i in 0..fish.len() {
            fish[i] -= 1;
            if fish[i] < 0 {
                fish[i] = 6;
                fish.push(8);
            }
        }
    }

    fish.len()
}

fn part2(fish: Vec<i8>) -> usize {
    let mut fish_new = vec![0usize; 7]; // Fish at 0 to 6 days
    let mut special_buffer = vec![0usize; 2]; // Fish at 7 and 8 days. These are moved into fish_new once they are in the "0 to 6" range

    for i in 0..7i8 {
        fish_new[i as usize] = fish.iter().filter(|&&n| n == i).count();
    }

    let mut base_line = 0usize;
    for _ in 0..256 {
        let entering = special_buffer[0];
        special_buffer[0] = special_buffer[1];
        special_buffer[1] = fish_new[base_line];
        fish_new[base_line] += entering;
        base_line = (base_line + 1) % 7;
    }

    fish_new.into_iter().chain(special_buffer.into_iter()).sum()
}