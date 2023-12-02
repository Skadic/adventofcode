#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl Cubes {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Cubes { red, green, blue }
    }

    pub fn max(self, other: Self) -> Self {
        Cubes::new(
            self.red.max(other.red),
            self.green.max(other.green),
            self.blue.max(other.blue),
        )
    }

    pub fn power(self) -> usize {
        self.red * self.green * self.blue
    }

    pub fn is_possible(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<Cubes>> {
    let mut games = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut game = vec![];
        let start = line.chars().position(|c| c == ':').unwrap() + 1;
        for game_cubes in line[start..].trim().split(';').map(str::trim) {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for cubes in game_cubes.split(',').map(str::trim) {
                let count = cubes
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                match cubes.split_whitespace().nth(1).unwrap() {
                    "red" => {
                        red += count;
                    }
                    "green" => {
                        green += count;
                    }
                    "blue" => {
                        blue += count;
                    }
                    _ => {}
                }
            }
            game.push(Cubes::new(red, green, blue))
        }
        games.push(game);
    }

    games
}
