use std::collections::HashSet;

use ansi_term::Color;
use day12::{process_part1, a_star, process_input};

fn main() {
    let input = include_str!("../../input.txt");


    let map = process_input(input);

    print!("Path: ");
    let path = a_star(&map).unwrap().into_iter().inspect(|&(x, y)| {
        let elevation = map.elevation(x, y);
        if elevation > 0 {
            let b = (elevation + b'a') as char;
            print!("{b}");
        }
    }).collect::<HashSet<_>>();
    println!("\nOf size: {}", path.len() - 1);
    println!("\n");

    for (y, line) in input.lines().map(str::trim).enumerate() {
        for (x, c) in line.chars().enumerate() {
            let elevation = map.elevation(x, y);
            if c == 'S' || c == 'E' {
                print!("{c}");
                continue;
            }
            if path.contains(&(x, y)) {
                print!("{}", Color::Fixed(elevation / (26 / 8) + 1).paint(&c.to_string()));
                continue;
            }
            print!("{}", Color::Fixed(8 + elevation.clamp(0, 5)).paint(&c.to_string()));
        }
        println!();
    }

    println!("Part 1: {}", process_part1(input) );
}