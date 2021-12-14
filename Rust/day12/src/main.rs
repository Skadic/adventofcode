use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../res/input.txt");

    let edges = input
        .lines()
        .map(|line| line.trim().split("-"))
        .map(|mut split| {
            (
                split.next().unwrap().to_owned(),
                split.next().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();

    let nodes = edges
        .iter()
        .cloned()
        .flat_map(|(a, b)| std::iter::once(a).chain(std::iter::once(b)))
        .collect::<HashSet<_>>();

    let adj = nodes
        .iter()
        .map(|node| {
            (
                node.clone(),
                edges
                    .iter()
                    .filter(|(a, b)| a == &node[..] || b == &node[..])
                    .map(|(a, b)| if b == node { a } else { b })
                    .cloned()
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    println!("Part 1: {:?}", part1(&adj));
    println!("Part 2: {:?}", part2(&adj));
}

fn part1(adj: &HashMap<String, HashSet<String>>) -> usize {
    let result = rec_search1(adj, "start", vec!["start"]);
    result.len()
}

fn part2(adj: &HashMap<String, HashSet<String>>) -> usize {
    let result = rec_search2(adj, "start", vec!["start"], None);
    result.len()
}

fn rec_search1<'a>(
    adj: &'a HashMap<String, HashSet<String>>,
    current: &'a str,
    path: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];

    for next in adj[current].iter() {
        match &next[..] {
            "end" => {
                paths.push(
                    path.iter()
                        .cloned()
                        .chain(std::iter::once("end"))
                        .collect::<Vec<_>>(),
                );
                continue;
            }
            "start" => {
                continue;
            }
            _ => {}
        }

        if &next.to_lowercase()[..] == next && path.contains(&&next[..]) {
            continue;
        }

        paths.extend(rec_search1(
            adj,
            &next[..],
            path.iter()
                .cloned()
                .chain(std::iter::once(&next[..]))
                .collect(),
        ))
    }

    paths
}

fn rec_search2<'a>(
    adj: &'a HashMap<String, HashSet<String>>,
    current: &'a str,
    path: Vec<&'a str>,
    twice: Option<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];

    for next in adj[current].iter() {
        let mut new_twice = twice;
        match &next[..] {
            "end" => {
                paths.push(
                    path.iter()
                        .cloned()
                        .chain(std::iter::once("end"))
                        .collect::<Vec<_>>(),
                );
                continue;
            }
            "start" => {
                continue;
            }
            _ => {}
        }

        if &next.to_lowercase()[..] == next && path.contains(&&next[..]) {
            if twice.is_some() || new_twice.is_some() {
                continue;
            }
            new_twice = Some(&next[..])
        }

        paths.extend(rec_search2(
            adj,
            &next[..],
            path.iter()
                .cloned()
                .chain(std::iter::once(&next[..]))
                .collect(),
            new_twice,
        ))
    }

    paths
}
