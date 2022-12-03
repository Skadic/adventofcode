use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    process_input_part1(input)
        .into_iter()
        .map(|(opponent, me)| {
            me.score()
                + if me.wins_against(opponent) {
                    6
                } else if me.is_draw(opponent) {
                    3
                } else {
                    0
                }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    process_input_part2(input)
        .into_iter()
        .map(|(opponent, desired_result)| (opponent, opponent.need_to_play(desired_result)))
        .map(|(opponent, me)| {
            me.score()
                + if me.wins_against(opponent) {
                    6
                } else if me.is_draw(opponent) {
                    3
                } else {
                    0
                }
        })
        .sum()
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl HandShape {
    fn need_to_play(self, desired_result: MatchResult) -> HandShape {
        use MatchResult::*;
        match desired_result {
            Win => self.shape_winning_against(),
            Draw => self,
            Loss => self.shape_losing_against()
        }
    }

    fn wins_against(self, other: HandShape) -> bool {
        other == self.shape_losing_against()
    }

    fn shape_losing_against(self) -> HandShape {
        use HandShape::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn shape_winning_against(self) -> HandShape {
        use HandShape::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn is_draw(self, other: HandShape) -> bool {
        self == other
    }

    fn score(self) -> usize {
        use HandShape::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl FromStr for HandShape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HandShape::*;
        Ok(match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => return Err(format!("invalid hand shape code: {s}")),
        })
    }
}

impl FromStr for MatchResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MatchResult::*;
        Ok(match s {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => return Err(format!("invalid match result code: {s}")),
        })
    }
}

fn process_input_part1(input: &str) -> Vec<(HandShape, HandShape)> {
    input
        .lines()
        .map(|line| {
            let mut split = line
                .trim()
                .split(" ")
                .filter_map(|token| token.parse().ok());
            let opponent_hand_shape = split.next().unwrap();
            let my_hand_shape = split.next().unwrap();
            (opponent_hand_shape, my_hand_shape)
        })
        .collect()
}

fn process_input_part2(input: &str) -> Vec<(HandShape, MatchResult)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.trim().split(" ");
            let opponent_hand_shape = split.next().unwrap().parse().unwrap();
            let my_hand_shape = split.next().unwrap().parse().unwrap();
            (opponent_hand_shape, my_hand_shape)
        })
        .collect()
}
