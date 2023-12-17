use priority_queue::PriorityQueue;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Axis {
    Horizontal,
    Vertical,
}
impl Axis {
    pub fn to_index(self) -> usize {
        use Axis::*;
        match self {
            Horizontal => 0,
            Vertical => 1,
        }
    }

    pub fn flip(self) -> Axis {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }

    pub fn dirs(self) -> [Direction; 2] {
        match self {
            Self::Horizontal => [Direction::Left, Direction::Right],
            Self::Vertical => [Direction::Up, Direction::Down],
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    pub fn go(self, x: usize, y: usize, n: usize, w: usize, h: usize) -> Option<(usize, usize)> {
        match self {
            Self::Left if x >= n => Some((x - n, y)),
            Self::Right if x + n < w => Some((x + n, y)),
            Self::Up if y >= n => Some((x, y - n)),
            Self::Down if y + n < h => Some((x, y + n)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct PathInfo {
    x: usize,
    y: usize,
    src_axis: Axis,
}

impl PathInfo {
    pub fn new(x: usize, y: usize, src_axis: Axis) -> Self {
        Self { x, y, src_axis }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rev<T>(T);

impl<T> Rev<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T> From<T> for Rev<T> {
    fn from(value: T) -> Self {
        Rev(value)
    }
}

pub fn dijkstra(map: &[Vec<usize>], start_x: usize, start_y: usize, min_step: usize, max_step: usize) -> usize {
    use Direction as D;
    let w = map[0].len();
    let h = map.len();
    let mut queue = PriorityQueue::new();

    let mut dist = vec![vec![[usize::MAX; 2]; w]; h];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let ph = PathInfo::new(x, y, Axis::Horizontal);
            let pv = PathInfo::new(x, y, Axis::Vertical);
            queue.push(ph, Rev(usize::MAX));
            queue.push(pv, Rev(usize::MAX));
        }
    }

    queue.change_priority(&PathInfo::new(start_x, start_y, Axis::Horizontal), Rev(0));
    queue.change_priority(&PathInfo::new(start_x, start_y, Axis::Vertical), Rev(0));
    dist[start_y][start_x][Axis::Horizontal.to_index()] = 0;
    dist[start_y][start_x][Axis::Vertical.to_index()] = 0;

    while !queue.is_empty() {
        let (PathInfo { x, y, src_axis }, cost) = queue.pop().unwrap();
        dist[y][x][src_axis.to_index()] = cost.into_inner();

        for dir in src_axis.flip().dirs() {
            for step in min_step..=max_step {
                let Some((new_x, new_y)) = dir.go(x, y, step, w, h) else {
                    continue;
                };
                let new_path = PathInfo::new(new_x, new_y, src_axis.flip());
                let Some(old_cost) = queue.get_priority(&new_path).copied() else {
                    continue;
                };

                let new_cost = cost.into_inner()
                    + (1..=step)
                        .map(|i| match dir {
                            D::Left => map[y][x - i],
                            D::Right => map[y][x + i],
                            D::Up => map[y - i][x],
                            D::Down => map[y + i][x],
                        })
                        .sum::<usize>();

                if new_cost < old_cost.into_inner() {
                    queue.change_priority(&new_path, new_cost.into());
                }
            }
        }
    }

    dist.last()
        .and_then(|l| l.last())
        .unwrap()
        .into_iter()
        .copied()
        .min()
        .unwrap()
}