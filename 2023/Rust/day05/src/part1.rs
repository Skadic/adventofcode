use day05::parse_input;

fn main() {
    let input = std::fs::read_to_string("../../inputs/day05/input.txt").unwrap();

    let (seeds, input) = parse_input(&input);

    let mut rng = seeds.into_iter().map(|v| v..=v).collect::<Vec<_>>();
    for i in 0..7 {
        rng = rng
            .into_iter()
            .flat_map(|v| input.map_range(i, v))
            .collect::<Vec<_>>();
    }
    println!("{}", rng.into_iter().map(|v| *v.start()).min().unwrap())
}
