use std::ops::RangeInclusive;

pub const INPUT: &str = include_str!("../input.txt");

pub mod part1;
pub mod part2;

pub const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Maps {
    maps: [Vec<RangeMap>; 7],
}

impl Maps {
    pub fn by_idx(&mut self, idx: usize) -> &mut Vec<RangeMap> {
        &mut self.maps[idx]
    }

    fn find_range_idx(&self, map_idx: usize, value: usize) -> usize {
        let maps = self.maps[map_idx].as_slice();
        let mut i = 0;
        while i < maps.len() && maps[i].src_start <= value {
            i += 1;
        }

        i.saturating_sub(1)
    }

    fn find_range(&self, map_idx: usize, value: usize) -> RangeMap {
        self.maps[map_idx][self.find_range_idx(map_idx, value)]
    }

    pub fn map_single(&self, mut seed: usize) -> usize {
        for i in 0..7 {
            let map = self.find_range(i, seed);
            seed = map.map(seed);
        }
        seed
    }

    pub fn map_range(
        &self,
        map_idx: usize,
        range: RangeInclusive<usize>,
    ) -> impl IntoIterator<Item = RangeInclusive<usize>> {
        let start = *range.start();
        let end = *range.end();
        let map = self.maps[map_idx].as_slice();
        let mut out = vec![];
        for &mapping in map {
            if mapping.src_start <= start && end <= mapping.src_end {
                out.push(mapping.map(start)..=mapping.map(end));
            } else if start <= mapping.src_start && mapping.src_end <= end {
                out.push(mapping.map(mapping.src_start)..=mapping.map(mapping.src_end));
            } else if start <= mapping.src_start && mapping.src_start <= end {
                out.push(mapping.map(mapping.src_start)..=mapping.map(end))
            } else if start <= mapping.src_end && mapping.src_end < end {
                out.push(mapping.map(start)..=mapping.map(mapping.src_end))
            }
        }
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeMap {
    src_start: usize,
    src_end: usize,
    offset: isize,
}

impl RangeMap {
    pub fn map(&self, seed: usize) -> usize {
        if self.contains(seed) {
            (seed as isize + self.offset) as usize
        } else {
            seed
        }
    }

    pub fn contains(&self, value: usize) -> bool {
        self.src_start <= value && value <= self.src_end
    }

    pub fn split(&self, idx: usize) -> (RangeMap, RangeMap) {
        (
            RangeMap {
                src_start: self.src_start,
                src_end: idx - 1,
                offset: self.offset,
            },
            RangeMap {
                src_start: idx,
                src_end: self.src_end,
                offset: self.offset,
            },
        )
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> (Vec<usize>, Maps) {
    let seeds = input.lines().next().unwrap()[6..]
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut maps = Maps::default();
    for (i, section) in input.split("\n\n").skip(1).enumerate() {
        let mut ranges = vec![];
        for line in section.trim().lines().skip(1) {
            let mut tokens = line.trim().split_whitespace();
            let dest_start = tokens.next().and_then(|s| s.parse::<isize>().ok()).unwrap();
            let src_start = tokens.next().and_then(|s| s.parse::<usize>().ok()).unwrap();
            let range_len = tokens.next().and_then(|s| s.parse::<usize>().ok()).unwrap();
            ranges.push(RangeMap {
                src_start,
                src_end: src_start + range_len,
                offset: dest_start - src_start as isize,
            });
        }
        *maps.by_idx(i) = ranges;
    }

    for i in 0..7 {
        let map = maps.by_idx(i);
        map.sort_by_key(|v| v.src_start);
        if map[0].src_start > 0 {
            let src_end = map[0].src_start;
            map.insert(
                0,
                RangeMap {
                    src_start: 0,
                    src_end,
                    offset: 0,
                },
            )
        }
        let mut j = 0;
        while j < map.len() - 1 {
            let l = map[j].src_end;
            let r = map[j + 1].src_start;
            if l < r {
                map.insert(
                    j,
                    RangeMap {
                        src_start: l,
                        src_end: r,
                        offset: 0,
                    },
                );
                j += 1;
            }
            j += 1;
        }
        map.push(RangeMap {
            src_start: map.last().unwrap().src_end,
            src_end: usize::MAX,
            offset: 0,
        })
    }
    (seeds, maps)
}
