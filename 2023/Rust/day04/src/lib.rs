use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub win: HashSet<usize>,
    pub my_cards: Vec<usize>,
}

impl Card {
    pub fn num_matches(&self) -> usize {
        self.my_cards
            .iter()
            .map(|nr| self.win.contains(nr))
            .filter(|&v| v)
            .count()
    }

    pub fn score(&self) -> usize {
        match self.num_matches() {
            0 => 0,
            n => 2usize.pow(n as u32 - 1),
        }
    }
}

fn parse_line((i, line): (usize, &str)) -> Card {
    let line = line[line.chars().position(|c| c == ':').unwrap() + 1..].trim();
    let (winning_str, my_cards_str) = {
        let mut segments = line.split('|').map(str::trim);
        let win = segments.next().unwrap();
        let my = segments.next().unwrap();
        (win, my)
    };

    Card {
        id: i,
        win: winning_str
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
        my_cards: my_cards_str
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
    }
}

pub fn parse_input(input: &str) -> Vec<Card> {
    input.lines().enumerate().map(parse_line).collect()
}
