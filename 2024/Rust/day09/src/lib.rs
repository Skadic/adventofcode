use std::fmt::Display;

use itertools::Itertools;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "2333133121414131402";
pub const SAMPLE2: &str = "233313312141413140242";

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Section {
    id: usize,
    length: usize,
    padding: usize,
}

impl Section {
    pub fn slice_off(&mut self, n: usize) -> Section {
        if n < self.length {
            self.length -= n;
            self.padding += n;
            Section {
                id: self.id,
                length: n,
                padding: 0,
            }
        } else {
            let old_len = self.length;
            self.length = 0;
            self.padding += old_len;
            Section {
                id: self.id,
                length: old_len,
                padding: 0,
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn padding(&self) -> usize {
        self.padding
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.padding == 0
    }

    pub fn reduce_padding(&mut self, n: usize) {
        self.padding -= n;
    }

    pub fn grow(&mut self, n: usize) {
        if self.padding < n {
            panic!("AAA")
        }
        self.padding -= n;
        self.length += n;
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::iter::repeat(char::from_digit(self.id as u32, 16).unwrap().to_string())
            .take(self.len())
            .chain(std::iter::repeat(".".to_owned()).take(self.padding))
            .collect::<String>();
        write!(f, "{s}")
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Section> {
    input
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(id, chunk)| {
            let mut chunk = chunk.filter_map(|c| c.to_digit(10));
            Section {
                id,
                length: chunk.next().unwrap() as usize,
                padding: chunk.next().unwrap_or(0) as usize,
            }
        })
        .collect()
}
