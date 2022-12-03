fn main() {
    let input = include_str!("../res/input.txt");

    let sequences = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    println!("Part 1: {}", part1(&sequences));
    println!("Part 2: {}", part2(&sequences));
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid bracket: {}", c),
    }
}

fn part1(sequences: &[&str]) -> usize {
    let mut first_error = vec![];

    for &sequence in sequences {
        let mut stack = vec![];

        for c in sequence.chars() {
            if "{([<".contains(c) {
                stack.push(closing(c));
            } else if c != stack.pop().unwrap() {
                first_error.push(match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("Invalid bracket: {}", c),
                });
                break;
            }
        }
    }

    first_error.into_iter().sum()
}

fn part2(sequences: &[&str]) -> usize {
    let mut completion_scores = vec![];

    'next_sequence: for &sequence in sequences {
        let mut stack = vec![];

        for c in sequence.chars() {
            if "{([<".contains(c) {
                stack.push(closing(c));
            } else if c != stack.pop().unwrap() {
                // Skip this invalid sequence
                continue 'next_sequence;
            }
        }

        completion_scores.push(
            stack
                .into_iter()
                .rev()
                .map(|c| ")]}>".chars().position(|c2| c == c2).unwrap() + 1)
                .fold(0, |score, value| score * 5 + value),
        );
    }

    completion_scores.sort_unstable_by(|a, b| a.cmp(&b));
    completion_scores[completion_scores.len() / 2]
}
