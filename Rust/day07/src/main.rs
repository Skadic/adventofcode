use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("res/input.txt").unwrap();

    let mut positions = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    positions.sort_unstable();

    println!("Part 1: {}", part1(&positions));
    println!("Part 2: {}", part2(&positions));
}

fn part1(positions: &[usize]) -> usize {
    // An ordered map that saves the values occurring in the positions and their first occurrence
    let mut transitions = BTreeMap::new();

    transitions.insert(positions[0], 0);
    for (i, &val) in positions.iter().enumerate() {
        transitions.entry(val).or_insert(i);
    }

    let mut fuel_costs = vec![positions.iter().map(|v| v - positions[0]).sum::<usize>()];
    let mut last_submarine_pos = positions[0];

    for (&submarine_pos, &i) in transitions.iter().skip(1) {
        let diff_to_previous = submarine_pos - last_submarine_pos;
        let last_cost = fuel_costs[fuel_costs.len() - 1];
        fuel_costs
            .push(last_cost + i * diff_to_previous - (positions.len() - i) * diff_to_previous);
        last_submarine_pos = submarine_pos;
    }

    fuel_costs.into_iter().min().unwrap()
}

fn part2(positions: &[usize]) -> usize {
    let mut transitions = BTreeMap::new();

    transitions.insert(positions[0], 0);
    for &val in positions.iter() {
        *transitions.entry(val).or_insert(0) += 1;
    }

    // For all different position values available
    transitions
        .iter()
        // Sum the fuel costs to align to this specific submarine position
        .map(|(&submarine_pos, _)| {
            transitions
                .iter()
                .map(|(&pos, &count)| {
                    count * gauss((submarine_pos as isize - pos as isize).abs() as usize)
                })
                .sum::<usize>()
        })
        // Get the minimum of these positions
        .min()
        .unwrap()
}

fn gauss(n: usize) -> usize {
    n * (n + 1) / 2
}
