fn main() {
    let input = std::fs::read_to_string("res/input.txt").unwrap();
    let bit_string_length = input.lines().next().unwrap().len();


    println!("Part 1: {:?}", part1(&input, bit_string_length));
    println!("Part 2: {:?}", part2(&input, bit_string_length));
}

fn part1(input: &str, bit_string_length: usize) -> usize {
    let gamma = input.chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .fold(vec![0isize; bit_string_length], |mut state, (i, c)| {
            state[i % bit_string_length] += if c == '0' { 1 } else { -1 };
            state
        }) // Create a vec, where each position contains a positive number, if there are more zeroes, or a negative number if there are more ones
        .into_iter()
        .map(|count| if count > 0 { 0usize } else { 1usize })
        .rev()
        .enumerate()
        .fold(0, |state, (i, value)| state | (value << i));
    
    let epsilon = !((usize::MAX << bit_string_length) | gamma);

    gamma * epsilon
}

fn part2(input: &str, bit_string_length: usize) -> usize {

    // Oxygen
    let mut leftover_values = input.lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();

    for i in 0..bit_string_length {
        let bit_count = leftover_values
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .fold(0isize, |state, c| state + if c == '1' { 1 } else { -1 });
    
        let frequent_bit = (bit_count >= 0) as u32;
        
        leftover_values = leftover_values.into_iter()
            .filter(|line| line.chars().nth(i).unwrap().to_digit(10).unwrap() == frequent_bit)
            .collect();

        if leftover_values.len() == 1 { 
            break
        }
    }

    let oxygen = usize::from_str_radix(&leftover_values[0], 2).unwrap();

    // CO2 
    let mut leftover_values = input.lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();

    for i in 0..bit_string_length {
        let bit_count = leftover_values
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .fold(0isize, |state, c| state + if c == '1' { 1 } else { -1 });
    
        let infrequent_bit = (bit_count < 0) as u32;
        
        leftover_values = leftover_values.into_iter()
            .filter(|line| line.chars().nth(i).unwrap().to_digit(10).unwrap() == infrequent_bit)
            .collect();

        if leftover_values.len() == 1 { 
            break
        }
    }

    let co2 = usize::from_str_radix(&leftover_values[0], 2).unwrap();

    oxygen * co2
}