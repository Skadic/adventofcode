use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, Read},
};

// 523 too high
// 492 is correct
// 461 too low

use tracing::info;

use crate::{
    dijkstra, dijkstra1, parse_input, Direction, DistType, INPUT, SAMPLE, SAMPLE2, SAMPLE3,
    SAMPLE4, SAMPLE5, SAMPLE6,
};

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);

    let mut dists = dijkstra(&input, 1, input.len() - 2);

    let end_x = dists[0].len() - 2;
    let end_y = 1;
    let end_dist = dists[end_y][end_x];
    match end_dist.0.cmp(&end_dist.1) {
        std::cmp::Ordering::Less => {
            dists[end_y][end_x].1 = DistType::MAX;
        }
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Greater => {
            dists[end_y][end_x].0 = DistType::MAX;
        }
    }
    for (y, row) in input.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            match v {
                '#' => print!("█"),
                '.' => print!(" "),
                _ => {}
            }
        }

        println!()
    }
    let best = backtrack(&input, &dists);

    println!("{:?}", dists[1][input[0].len() - 2]);

    for (y, row) in dists.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if best.contains(&(x, y)) {
                print!("·")
            } else {
                match input[y][x] {
                    '#' => print!("█"),
                    '.' => print!(" "),
                    _ => {}
                }
            }
        }

        println!()
    }

    let mut buf = String::new();
    while let Ok(i) = std::io::stdin().lock().read_line(&mut buf) {
        if i == 0 {
            break;
        }
        let Some((qx, qy)) = buf.split_once(",").and_then(|(l, r)| {
            Option::zip(
                l.trim().parse::<usize>().ok(),
                r.trim().parse::<usize>().ok(),
            )
        }) else {
            buf.clear();
            continue;
        };
        for (y, row) in dists.iter().enumerate() {
            for (x, &v) in row.iter().enumerate() {
                if x == qx && y == qy {
                    print!("X");
                } else if best.contains(&(x, y)) {
                    print!("·")
                } else {
                    match input[y][x] {
                        '#' => print!("█"),
                        '.' => print!(" "),
                        _ => {}
                    }
                }
            }

            println!()
        }
        info!(dist = ?dists[qy][qx]);
        buf.clear();
    }
    info!(result = best.len());

    Ok(())
}

fn backtrack1(map: &[Vec<char>], dists: &[Vec<DistType>]) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back((map[0].len() - 2, 1, false, true));
    let mut visited = HashSet::new();

    while let Some((current_x, current_y, can_continue_horiz, can_continue_vert)) = queue.pop_back()
    {
        visited.insert((current_x, current_y));
        let current_dist = dists[current_y][current_x];
        queue.extend(
            [
                (current_x.wrapping_sub(1), current_y, Direction::Left),
                (current_x + 1, current_y, Direction::Right),
                (current_x, current_y.wrapping_sub(1), Direction::Up),
                (current_x, current_y + 1, Direction::Down),
            ]
            .into_iter()
            .filter_map(|(x, y, current_move)| {
                let neighbor_dist = dists[y][x];
                if visited.contains(&(x, y)) {
                    return None;
                }

                let (can_continue_horiz, can_continue_vert) = match current_move {
                    Direction::Left | Direction::Right if can_continue_horiz => {
                        let can_continue_horiz =
                            neighbor_dist != DistType::MAX && neighbor_dist + 1 == current_dist;
                        let can_continue_vert =
                            neighbor_dist != DistType::MAX && neighbor_dist + 1001 == current_dist;
                        (can_continue_horiz, can_continue_vert)
                    }
                    Direction::Up | Direction::Down if can_continue_vert => {
                        let can_continue_horiz =
                            neighbor_dist != DistType::MAX && neighbor_dist + 1001 == current_dist;
                        let can_continue_vert =
                            neighbor_dist != DistType::MAX && neighbor_dist + 1 == current_dist;
                        (can_continue_horiz, can_continue_vert)
                    }
                    _ => return None,
                };
                if !can_continue_horiz && !can_continue_vert {
                    return None;
                }
                Some((x, y, can_continue_horiz, can_continue_vert))
            }),
        );
    }

    visited
}

fn backtrack(map: &[Vec<char>], dists: &[Vec<(DistType, DistType)>]) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back((map[0].len() - 2, 1, Direction::Down, false, true));
    let mut visited = HashSet::new();
    let normalize = |dir: Direction| {
        if dir.is_horiz() {
            Direction::Right
        } else {
            Direction::Up
        }
    };

    while let Some((current_x, current_y, last_move, can_continue_horiz, can_continue_vert)) =
        queue.pop_back()
    {
        visited.insert((current_x, current_y, normalize(last_move)));
        let (current_dist_horiz, current_dist_vert) = dists[current_y][current_x];
        queue.extend(
            [
                (current_x.wrapping_sub(1), current_y, Direction::Left),
                (current_x + 1, current_y, Direction::Right),
                (current_x, current_y.wrapping_sub(1), Direction::Up),
                (current_x, current_y + 1, Direction::Down),
            ]
            .into_iter()
            .filter_map(|(x, y, current_move)| {
                let (neighbor_dist_horiz, neighbor_dist_vert) = dists[y][x];
                if visited.contains(&(x, y, normalize(current_move))) {
                    return None;
                }

                let (can_continue_horiz, can_continue_vert) = match current_move {
                    Direction::Left | Direction::Right if can_continue_horiz => {
                        let can_continue_horiz = neighbor_dist_horiz != DistType::MAX
                            && neighbor_dist_horiz + 1 == current_dist_horiz;
                        let can_continue_vert = neighbor_dist_vert != DistType::MAX
                            && neighbor_dist_vert + 1001 == current_dist_horiz;
                        (can_continue_horiz, can_continue_vert)
                    }
                    Direction::Up | Direction::Down if can_continue_vert => {
                        let can_continue_horiz = neighbor_dist_horiz != DistType::MAX
                            && neighbor_dist_horiz + 1001 == current_dist_vert;
                        let can_continue_vert = neighbor_dist_vert != DistType::MAX
                            && neighbor_dist_vert + 1 == current_dist_vert;
                        (can_continue_horiz, can_continue_vert)
                    }
                    _ => return None,
                };

                if x == 13 && y == 9 {
                    info!(x, y, can_continue_horiz, can_continue_vert);
                }
                if !can_continue_horiz && !can_continue_vert {
                    return None;
                }
                Some((x, y, current_move, can_continue_horiz, can_continue_vert))
            }),
        );
    }

    visited.into_iter().map(|(x, y, _)| (x,y)).collect()
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
