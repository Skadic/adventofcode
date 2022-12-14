use std::{cmp::Reverse, collections::BinaryHeap, hash::Hash, ops::Deref, path};

use priority_queue::PriorityQueue;

#[derive(Debug, PartialEq, Eq)]
pub struct CopyGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> CopyGrid<T> {
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[y * self.width + x]
    }

    fn entries(&self) -> impl Iterator<Item = ((usize, usize), T)> + '_ {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .map(move |(x, y)| ((x, y), self.data[y * self.width() + x]))
    }
}

impl<T> CopyGrid<T> {
    #[inline]
    fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self {
            data: (0..width * height).map(|_| Default::default()).collect(),
            width,
            height,
        }
    }

    #[inline]
    fn with_data(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, v: T) -> T {
        let w = self.width();
        std::mem::replace(&mut self.data[y * w + x], v)
    }

    #[inline]
    pub fn get_ref(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    #[inline]
    fn width(&self) -> usize {
        self.width
    }

    #[inline]
    fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    elevations: CopyGrid<u8>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

impl Deref for Map {
    type Target = CopyGrid<u8>;

    fn deref(&self) -> &Self::Target {
        &self.elevations
    }
}

impl Map {
    #[inline]
    pub fn elevation(&self, x: usize, y: usize) -> u8 {
        self.elevations.get(x, y)
    }

    pub fn start_pos(&self) -> (usize, usize) {
        self.start_pos
    }

    pub fn end_pos(&self) -> (usize, usize) {
        self.end_pos
    }

    pub fn width(&self) -> usize {
        self.elevations.width()
    }

    pub fn height(&self) -> usize {
        self.elevations.height()
    }
}

pub fn process_input(input: &str) -> Map {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().trim().len();
    let mut elevations = Vec::with_capacity(input.len());
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    for (y, line) in input.lines().map(str::trim).enumerate() {
        for (x, c) in line.chars().enumerate() {
            elevations.push(match c {
                'S' => {
                    start_pos = (x, y);
                    0
                }
                'E' => {
                    end_pos = (x, y);
                    25
                }
                c => {
                    let c = c as u8;
                    c - b'a'
                }
            });
        }
    }

    Map {
        elevations: CopyGrid::with_data(elevations, width, height),
        start_pos,
        end_pos,
    }
}

/// Priority for the priority queue in A*
#[derive(PartialEq, Debug)]
struct Priority(f64);
impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.0)
            .partial_cmp(&Reverse(other.0))
            .or(Some(std::cmp::Ordering::Equal))
    }
}
impl Eq for Priority {}
impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn a_star(map: &Map) -> Option<Vec<(usize, usize)>> {
    let start @ (start_x, start_y) = map.start_pos();
    let end @ (end_x, end_y) = map.end_pos();

    // Manhattan distance from the given point to the end position
    let dist = move |(x, y): (usize, usize)| {
        let x = x as f64 - end_x as f64;
        let y = y as f64 - end_y as f64;
        x.abs() + y.abs()
    };

    // Estimate the total distance to the end node
    let estimate = move |best_path_len: &CopyGrid<Option<usize>>, pos: (usize, usize)| {
        Priority(dist(pos) + best_path_len.get(pos.0, pos.1).unwrap_or(0) as f64)
    };

    // The closed list. A position is true if that position has been fully explored and the shortest path been found
    let mut closed = CopyGrid::<bool>::new(map.width(), map.height());
    // This contains the length of the current best path to this position
    let mut best_path_length = CopyGrid::<Option<usize>>::new(map.width(), map.height());
    best_path_length.set(start_x, start_y, Some(0));
    // This contains the predecessor of this position in the shortest path to itself
    let mut predecessor = CopyGrid::<(usize, usize)>::new(map.width(), map.height());
    predecessor.set(start_x, start_y, start);
    // A position is true, if this nodes has been seen before
    let mut known_node_grid = CopyGrid::<bool>::new(map.width(), map.height());
    known_node_grid.set(start_x, start_y, true);
    // Conatins the currently known but unprocessed position, with the top of the heap being position with the smallest expected distance from the goal
    let mut known_nodes = PriorityQueue::<(usize, usize), Priority>::new();
    known_nodes.push(start, Priority(dist(start)));
    // Vector for storing the neighbors of a position
    let mut neighbors = Vec::with_capacity(4);

    loop {
        //println!("{known_nodes:?}");
        let Some((current@(current_x, current_y), Priority(_))) = known_nodes.pop() else {
            return None;
        };
        let current_elevation = map.elevation(current_x, current_y);
        //println!("choosing current: {current:?} with elevation {current_elevation}");

        closed.set(current_x, current_y, true);

        {
            let current_x = current_x as isize;
            let current_y = current_y as isize;

            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .map(|(delta_x, delta_y)| (current_x + delta_x, current_y + delta_y))
                .filter(|&(x, y)| {
                    x == x.clamp(0, map.width() as isize - 1)
                        && y == y.clamp(0, map.height() as isize - 1)
                })
                .for_each(|(x, y)| neighbors.push((x as usize, y as usize)))
        }

        let path_to_next = best_path_length.get(current_x, current_y).unwrap();

        for neighbor @ (neighbor_x, neighbor_y) in neighbors.drain(..) {
            let neighbor_elevation = map.elevation(neighbor_x, neighbor_y);
            //println!("neighbor: {neighbor:?} with elevation {neighbor_elevation}");

            if current_elevation < neighbor_elevation && neighbor_elevation - current_elevation > 1
            {
                continue;
            }

            if closed.get(neighbor_x, neighbor_y) {
                continue;
            }

            // If we already have recorded a path for this neighbor and that path is shorter
            // don't update it
            if let Some(len) = best_path_length.get(neighbor_x, neighbor_y) {
                if path_to_next + 1 >= len {
                    continue;
                }
            }

            best_path_length.set(neighbor_x, neighbor_y, Some(path_to_next + 1));
            predecessor.set(neighbor_x, neighbor_y, current);

            if !known_node_grid.get(neighbor_x, neighbor_y) {
                known_node_grid.set(neighbor_x, neighbor_y, true);
                known_nodes.push(neighbor, estimate(&best_path_length, neighbor));
            } else {
                known_nodes.change_priority(&neighbor, estimate(&best_path_length, neighbor));
            }
        }

        // In this case we're done
        if current == end {
            break;
        }
    }

    let mut path = vec![end];
    let mut current = end;
    while current != predecessor.get(current.0, current.1) {
        current = predecessor.get(current.0, current.1);
        path.push(current);
    }
    path.reverse();

    Some(path)
}

pub fn process_part1(input: &str) -> usize {
    let map = process_input(input);
    let path_len = a_star(&map)
        .as_deref()
        .map(<[_]>::len)
        .map(|len| len - 1)
        .unwrap_or(0);
    path_len
}

pub fn process_part2(input: &str) -> usize {
    let mut map = process_input(input);
    let entries = map.entries().collect::<Vec<_>>();
    entries
        .into_iter()
        .filter_map(
            |(pos, elevation)| {
                if elevation == 0 {
                    Some(pos)
                } else {
                    None
                }
            },
        )
        .filter_map(|start| {
            map.start_pos = start;
            a_star(&map).as_deref().map(<[_]>::len).map(|len| len - 1)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{a_star, process_input, process_part1};

    #[test]
    fn test_part_1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let map = process_input(input);
        assert_eq!(
            31,
            a_star(&map)
                .as_deref()
                .map(<[_]>::len)
                .map(|len| len - 1)
                .unwrap_or(0)
        );
    }
}
