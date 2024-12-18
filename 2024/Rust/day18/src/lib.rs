use std::collections::{HashMap, HashSet, VecDeque};

pub const INPUT: (&str, usize) = (include_str!("../input.txt"), 71);
pub const SAMPLE: (&str, usize) = (
    "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
    7,
);

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input((input, size): (&str, usize)) -> (Vec<(usize, usize)>, Vec<Vec<char>>) {
    (
        input
            .lines()
            .filter_map(|line| {
                line.split_once(",")
                    .and_then(|(x, y)| Option::zip(x.parse().ok(), y.parse().ok()))
            })
            .collect(),
        (0..size).map(|_| vec!['.'; size]).collect(),
    )
}

// yup, it's dfs again
fn bfs(chars: &[Vec<char>], x: usize, y: usize) -> HashMap<(usize, usize), usize> {
    let mut queue = VecDeque::new();
    queue.push_back((x, y, 0usize));
    let mut visited = HashMap::new();
    visited.insert((x, y), 0);

    while let Some((current_x, current_y, dist)) = queue.pop_front() {
        [
            (current_x.wrapping_sub(1), current_y),
            (current_x + 1, current_y),
            (current_x, current_y.wrapping_sub(1)),
            (current_x, current_y + 1),
        ]
        .into_iter()
        .filter(|pos| !visited.contains_key(pos))
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|(x, y)| {
            chars
                .get(y)
                .and_then(|row| row.get(x))
                .map(|&v| ((x, y), v))
        })
        .filter(|&(_, v)| v != '#')
        .map(|((x, y), _)| (x, y, dist + 1))
        .for_each(|v@(x, y, d)| {
            queue.push_back(v);
            visited.insert((x, y), d);
        });
    }

    visited
}
