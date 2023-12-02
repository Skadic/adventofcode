use day02::{parse_input, Cubes};

fn main() {
    let input = std::fs::read_to_string("../../inputs/day02/input.txt").unwrap();
    let input = parse_input(&input);

    let v = input
        .into_iter()
        .map(|game| {
            game.into_iter()
                .fold(Cubes::new(0, 0, 0), Cubes::max)
        })
        .enumerate()
        .filter(|(_, cubes)| cubes.is_possible(&Cubes::new(12, 13, 14)))
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("part 1: {v}")
}
