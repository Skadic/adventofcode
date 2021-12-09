use std::{
    cell::RefCell,
    collections::{BTreeSet, HashSet, VecDeque},
    vec,
};

fn main() {
    let input = include_str!("../res/input.txt");

    let map = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn low_points(map: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut low_points = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let current = map[y][x];

            let is_low_point = [0, 2]
                .into_iter()
                .flat_map(|i| {
                    [
                        map.get(y.wrapping_sub(1).wrapping_add(i)).map(|row| row[x]),
                        map[y].get(x.wrapping_sub(1).wrapping_add(i)).cloned(),
                    ]
                })
                .filter_map(|opt| opt)
                .all(|neighbor| current < neighbor);

            if is_low_point {
                low_points.push((x, y));
            }
        }
    }
    low_points
}

fn part1(map: &[Vec<usize>]) -> usize {
    low_points(map)
        .into_iter()
        .map(|(x, y)| map[y][x] + 1)
        .sum()
}

fn part2(map: &[Vec<usize>]) -> usize {
    // The low points on the map. these will always be a subset of leftover_positions
    let mut low_points = low_points(map);

    // All positions that are up for grabs and have not been visited before. Every position except positions with value 9
    let leftover_points = RefCell::new({
        let mut set = HashSet::new();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] < 9 {
                    set.insert((x, y));
                }
            }
        }
        set
    });

    // Returns all neighbors of a given position, that are still available (i.e. are found in 'leftover_points')
    let leftover_neigbors = |x: usize, y: usize| {
        [0, 2]
            .into_iter()
            .flat_map(|i| {
                [
                    (x, y.wrapping_sub(1).wrapping_add(i)),
                    (x.wrapping_sub(1).wrapping_add(i), y),
                ]
            })
            .filter(|neighbor| leftover_points.borrow().contains(neighbor))
            .collect::<BTreeSet<(usize, usize)>>()
    };

    // All basins are saved here
    let mut basins = vec![];

    // While there are still low points available
    while let Some(pos) = low_points.pop() {
        let mut basin = HashSet::new();

        // All positions that are part of the (tentative) border of the basin and have not been visited. From these positions, the basin can grow
        let mut border = VecDeque::new();
        // The basin starts as just the original position
        border.push_back(pos);

        // Remove the position from the low_points vec and leftover_points, since it has been visited now
        if let Some(pos_index) = low_points.iter().position(|p| p == &pos) {
            low_points.remove(pos_index);
        }
        leftover_points.borrow_mut().remove(&pos);

        // Take a position out of the border, grow the basin by the position's neighbors and remove the neighbors from low_points and leftover_positions
        // Repeat this until the basin can't grow any more (i.e. the border is empty)
        while let Some(border_pos @ (border_x, border_y)) = border.pop_front() {
            basin.insert(border_pos);

            let neighbors = leftover_neigbors(border_x, border_y);
            for neighbor in neighbors {
                if let Some(pos_index) = low_points.iter().position(|p| p == &neighbor) {
                    low_points.remove(pos_index);
                }
                leftover_points.borrow_mut().remove(&neighbor);

                border.push_back(neighbor);
            }
        }
        // add the finished basin to the basins vec
        basins.push(basin);
    }

    // Sort the basins so the largest basins are at the front
    basins.sort_unstable_by(|b1, b2| b2.len().cmp(&b1.len()));

    // Multiply the size of the largest 3 basins
    basins.into_iter().take(3).map(|set| set.len()).product()
}
