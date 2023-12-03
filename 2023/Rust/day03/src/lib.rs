#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
    Number(usize),
    Symbol,
    Gear,
    Dot,
}

impl Entry {
    pub fn is_symbol(self) -> bool {
        matches!(self, Self::Symbol | Self::Gear)
    }

    pub fn is_num(self) -> bool {
        matches!(self, Self::Number(_))
    }
}

impl From<char> for Entry {
    fn from(value: char) -> Self {
        match value {
            c if value.is_ascii_digit() => Self::Number(c.to_digit(10).unwrap() as usize),
            '.' => Self::Dot,
            '*' => Self::Gear,
            _ => Self::Symbol,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<Entry>> {
    let mut input_vec = vec![];
    for line in input.lines() {
        let line = line.trim();
        input_vec.push(line.chars().map(Entry::from).collect::<Vec<_>>());
    }

    input_vec
}

pub fn num_is_adjacent(input: &[Vec<Entry>], row: usize, col: usize) -> bool {
    let start_y = row.saturating_sub(1);
    let end_y = (row + 1).min(input.len() - 1);
    let start_x = col.saturating_sub(1);
    let mut end_x = col.min(input[row].len() - 1);
    while end_x < input[row].len() && input[row][end_x].is_num() {
        end_x += 1
    }
    end_x = end_x.min(input[row].len() - 1);

    (start_y..=end_y)
        .flat_map(|row| &input[row][start_x..=end_x])
        .any(|&v| v.is_symbol())
}

pub fn get_num(input: &[Vec<Entry>], row: usize, mut col: usize) -> usize {
    let mut digits = vec![];
    while col < input[row].len() {
        if let Entry::Number(v) = input[row][col] {
            digits.push(v)
        } else {
            break;
        }
        col += 1
    }
    digits.into_iter().fold(0, |acc, new| (acc * 10) + new)
}

pub fn adjacent_to_two(input: &[Vec<Entry>], row: usize, mut col: usize) -> bool {
    let start_y = row.saturating_sub(1);
    let end_y = (row + 1).min(input.len() - 1);
    let start_x = col.saturating_sub(1);
    let end_x = (col + 1).min(input[row].len() - 1);

    let mut num_numbers = 0;
    for y in start_y..=end_y {
        let mut last_was_num = false;
        for x in start_x..=end_x {
            if !last_was_num && input[y][x].is_num() {
                last_was_num = true;
                num_numbers += 1;
            }
            last_was_num = input[y][x].is_num()
        }
    }
    num_numbers == 2
}

pub fn gear_ratio(input: &[Vec<Entry>], row: usize, col: usize) -> usize {
    let start_y = row.saturating_sub(1);
    let end_y = (row + 1).min(input.len() - 1);
    let start_x = col.saturating_sub(1);
    let end_x = (col + 1).min(input[row].len() - 1);

    let mut n1 = None;
    let mut n2 = None;
    for y in start_y..=end_y {
        let mut last_was_num = false;
        for mut x in start_x..=end_x {
            if !last_was_num && input[y][x].is_num() {
                if x == start_x {
                    while input[y][x].is_num() && x > 0 {
                        x = x.saturating_sub(1);
                    }
                    if !input[y][x].is_num() {
                        x += 1;
                    }
                }
                if n1.is_none() {
                    n1 = Some(get_num(input, y, x));
                } else {
                    n2 = Some(get_num(input, y, x));
                }
            }
            last_was_num = input[y][x].is_num()
        }
    }

    n1.unwrap() * n2.unwrap()
}
