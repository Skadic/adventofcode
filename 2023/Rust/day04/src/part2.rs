use day04::{parse_input, Card};

fn main() {
    let input = std::fs::read_to_string("../../inputs/day04/input.txt").unwrap();
    let input = parse_input(&input);
    println!("part 2: {}", copies(&input).into_iter().sum::<usize>());
}

fn copies(cards: &[Card]) -> Vec<usize> {
    let mut copies = vec![0; cards.len()];
    for card in cards {
        let num_matches = card.num_matches();
        for i in 1..=num_matches {
            match card.id + i < cards.len() {
                true => copies[card.id + i] += 1,
                false => break,
            }
        }
    }

    for card_id in 0..cards.len() {
        let num_matches = cards[card_id].num_matches();
        for i in 1..=num_matches {
            match card_id + i < cards.len() {
                true => copies[card_id + i] += copies[card_id],
                false => break,
            }
        }
    }

    copies.iter_mut().for_each(|v| *v += 1);

    copies
}
