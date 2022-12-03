
#[derive(Default, Debug)]
struct Submarine {
    pos: isize,
    depth: isize,
    aim: isize,
}

fn main() {
    let instructions: Vec<(String, isize)> = std::fs::read_to_string("res/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.trim().split(" ");
            (split.next().unwrap().to_owned(), split.next().and_then(|v| v.parse().ok()).unwrap())
        })
        .collect();

    let sub = part1(&instructions);
    println!("Part 1: {:?}", sub.pos * sub.depth);
    
    let sub = part2(&instructions);
    println!("Part 2: {:?}", sub.pos * sub.depth)
}

fn part1(instructions: &[(String, isize)]) -> Submarine {
    let mut submarine = Submarine::default();

    for (instruction, value) in instructions {
        match &instruction[..] {
            "forward"   => submarine.pos += value,
            "down"      => submarine.depth += value,
            "up"        => submarine.depth -= value,
            _ => {}
        }
    }

    submarine
}

fn part2(instructions: &[(String, isize)]) -> Submarine {
    let mut submarine = Submarine::default();

    for (instruction, value) in instructions {
        match &instruction[..] {
            "forward"   => {submarine.pos += value; submarine.depth += value * submarine.aim},
            "down"      => submarine.aim += value,
            "up"        => submarine.aim -= value,
            _ => {}
        }
    }

    submarine
}


