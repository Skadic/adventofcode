
#[derive(Debug, Clone, PartialEq)]
struct Board(Vec<Vec<(usize, bool)>>);

impl Board {
    pub fn mark(&mut self, num: usize) {
        if let Some(reference) = self
            .0
            .iter_mut()
            .flatten()
            .filter(|(v, _)| *v == num)
            .next()
        {
            *reference = (num, true);
        }
    }

    pub fn has_won(&self) -> bool {
        if self.0.iter().any(|row| row.iter().all(|tup| tup.1)) {
            return true;
        }

        for i in 0..5 {
            if self.0.iter().all(|row| row[i].1) {
                return true;
            }
        }

        false
    }

    pub fn score(&self, value: usize) -> usize {
        self.0.iter()
            .flatten()
            .filter_map(|(v, b)| if !b { Some(*v) } else { None })
            .sum::<usize>() * value
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        Board(
            input
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| (num.parse().unwrap(), false))
                        .collect()
                })
                .collect(),
        )
    }
}

fn main() {
    let input = std::fs::read_to_string("res/input.txt").unwrap();
    let sequence = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let boards_string = input.lines().skip(1).collect::<Vec<_>>();

    let mut boards = boards_string
        .chunks(6)
        .map(|slice| Board::from(&slice.join("\n")[1..]))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&sequence, &mut (boards.clone())));
    println!("Part 2: {}", part2(&sequence, &mut boards));
    

}

fn part1(sequence: &[usize], boards: &mut [Board]) -> usize {

    for &value in sequence {
        for board in boards.iter_mut() {
            board.mark(value);
        }
        for board in boards.iter_mut() {
            if board.has_won() {
                return board.score(value)
            }
        }
    }

    0
}

fn part2(sequence: &[usize], boards: &mut Vec<Board>) -> usize {

    let mut vec = boards.clone();

    for &value in sequence {
        
        let mut i = 0usize;
        while i < vec.len() {
            vec[i].mark(value);

            let mut next_i = i + 1;
            
            if vec[i].has_won() {
                if vec.len() == 1 {
                    return vec[0].score(value);
                } else {
                    vec.remove(i);
                    next_i -= 1;
                }
            }
            i = next_i;
        }
    }
    
    0
}