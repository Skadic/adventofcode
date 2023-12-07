use std::cmp::Ordering;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Play {
    cards: [Card; 5],
    bid: usize,
}

impl Play {
    #[tracing::instrument]
    fn get_hand(hand: [Card; 5]) -> Hand {
        let mut types = [0; 13];
        for card in hand {
            let idx = match card {
                Card::Number(i) => i - 2,
                Card::T => 8,
                Card::J => 9,
                Card::Q => 10,
                Card::K => 11,
                Card::A => 12,
            };
            types[idx as usize] += 1;
        }
        let (max_idx, max) = types
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_, v)| v)
            .unwrap();
        types[max_idx] = 0;
        let (max_idx2, max2) = types
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_, v)| v)
            .unwrap();
        match max {
            5 => Hand::FiveOfAKind(Card::from(max_idx)),
            4 => Hand::FourOfAKind(Card::from(max_idx)),
            3 if max2 == 2 => Hand::FullHouse(Card::from(max_idx), Card::from(max_idx2)),
            3 => Hand::ThreeOfAKind(Card::from(max_idx)),
            2 if max2 == 2 => Hand::TwoPair(
                Card::from(max_idx.max(max_idx2)),
                Card::from(max_idx.min(max_idx2)),
            ),
            2 => Hand::OnePair(Card::from(max_idx)),
            1 => Hand::HighCard(Card::from(max_idx)),
            _ => panic!("invalid maximum: {max}"),
        }
    }

    #[tracing::instrument]
    fn upgrade(cards: [Card; 5]) -> [Card; 5] {
        let mut type_exists = [false; 13];
        for t in cards {
            type_exists[t.to_int()] = true;
        }
        if !type_exists[Card::J.to_int()] {
            return cards;
        }
        type_exists[Card::J.to_int()] = false;
        let mut current = cards;

        let j_pos = cards.iter().position(|&v| v == Card::J).unwrap();

        let mut local_cards = current;
        for (ty, exists) in type_exists.iter().enumerate() {
            if !exists {
                continue;
            }
            local_cards[j_pos] = ty.into();
            let upgraded = Self::upgrade(local_cards);
            let h1 = Self::get_hand(upgraded);
            let h2 = Self::get_hand(current);

            match h1.cmp(&h2) {
                Ordering::Greater => current = upgraded,
                Ordering::Equal => match Self::card_cmp(&upgraded, &current) {
                    Ordering::Greater => current = upgraded,
                    _ => {}
                },
                _ => {}
            }
        }

        current
    }

    fn cmp_lex(&self, other: &Play) -> Ordering {
        let t = Self::get_hand(self.cards);
        let ot = Self::get_hand(other.cards);
        match t.cmp(&ot) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            o => o,
        }
    }

    #[tracing::instrument(skip(self, other))]
    fn cmp_2nd(&self, other: &Play) -> Ordering {
        let cards = Self::upgrade(self.cards);
        let other_cards = Self::upgrade(other.cards);
        let t = Self::get_hand(cards);
        let ot = Self::get_hand(other_cards);
        match t.cmp(&ot) {
            Ordering::Equal => Self::card_cmp(&self.cards, &other.cards),
            o => o,
        }
    }

    fn card_cmp(l: &[Card; 5], r: &[Card; 5]) -> Ordering {
        for (&l, &r) in l.iter().zip(r) {
            match (l, r) {
                _ if l == r => {}
                (Card::J, _) => return Ordering::Less,
                (_, Card::J) => return Ordering::Greater,
                _ => return l.cmp(&r),
            }
        }
        Ordering::Equal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Hand {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

impl Hand {
    fn to_int(self) -> usize {
        use Hand::*;
        match self {
            HighCard(_) => 0,
            OnePair(_) => 1,
            TwoPair(_, _) => 2,
            ThreeOfAKind(_) => 3,
            FullHouse(_, _) => 4,
            FourOfAKind(_) => 5,
            FiveOfAKind(_) => 6,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_int().partial_cmp(&other.to_int())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Card {
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn to_int(self) -> usize {
        use Card::*;
        match self {
            Number(i) => (i - 2) as usize,
            T => 8,
            J => 9,
            Q => 10,
            K => 11,
            A => 12,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_int().partial_cmp(&other.to_int())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            c if c.is_ascii_digit() => Self::Number(c.to_digit(10).unwrap() as u8),
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("invalid char: {value}"),
        }
    }
}

impl From<usize> for Card {
    fn from(value: usize) -> Self {
        match value {
            i if i < 8 => Self::Number(i as u8 + 2),
            8 => Self::T,
            9 => Self::J,
            10 => Self::Q,
            11 => Self::K,
            12 => Self::A,
            _ => panic!("invalid usize: {value}"),
        }
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Play> {
    input
        .lines()
        .map(str::trim)
        .map(|l| l.split_once(" ").unwrap())
        .map(|(card_str, bid_str)| {
            let mut cards = [Card::A; 5];
            for (i, c) in card_str.char_indices() {
                cards[i] = c.into();
            }
            let bid = bid_str.parse().unwrap();
            Play { cards, bid }
        })
        .collect()
}
