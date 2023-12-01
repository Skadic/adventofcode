pub fn parse_input_1(path: &str) -> std::io::Result<usize> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            (
                line.chars()
                    .find_map(|c| c.to_digit(10).map(|c| c as usize))
                    .unwrap(),
                line.chars()
                    .rev()
                    .find_map(|c| c.to_digit(10).map(|c| c as usize))
                    .unwrap(),
            )
        })
        .map(|(l, r)| l * 10 + r)
        .sum())
}

fn parse_any_num(token: &str) -> Option<usize> {
    (1..=5.min(token.len())).find_map(|len| {
        Some(match token[0..len].trim() {
            c if c.len() == 1 => {
                if let Ok(v) = c.parse() {
                    v
                } else {
                    return None;
                }
            }
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => return None,
        })
    })
}

pub fn parse_input_2(path: &str) -> std::io::Result<usize> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            (
                (0..line.len())
                    .find_map(|i| parse_any_num(&line[i..]))
                    .unwrap(),
                (0..line.len())
                    .rev()
                    .find_map(|i| parse_any_num(&line[i..]))
                    .unwrap(),
            )
        })
        .map(|(l, r)| l * 10 + r)
        .sum())
}
