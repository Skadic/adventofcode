use std::{
    collections::{HashMap, HashSet},
};

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

    println!("{:?}", part1(&adj));
}

fn part1(adj: &HashMap<String, HashSet<String>>) -> usize {
    let result = rec_search(adj, "start", vec!["start"]);
    result.len()
}

fn rec_search<'a>(
    adj: &'a HashMap<String, HashSet<String>>,
    current: &'a str,
    path: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];

    for next in adj[current].iter() {
        print!("{:?} -> {} ", path, next);
        match &next[..] {
            "end" => {
                paths.push(
                    path.iter()
                        .cloned()
                        .chain(std::iter::once("end"))
                        .collect::<Vec<_>>(),
                );
                println!("O");
                continue;
            }
            "start" => {
                println!("X");
                continue;
            }
            _ => {}
        }

        if &next.to_lowercase()[..] == next
            && path
                .iter()
                .any(|&node| node != "start" && node.to_lowercase() == node)
        {
            println!("X");
            continue;
        }
        println!("C");

        paths.extend(rec_search(
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
