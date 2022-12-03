/// Line with attributes x1, y1, x2, y2
struct Line(usize, usize, usize, usize);

impl Line {
    pub fn is_aligned(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn is_horizontal(&self) -> bool {
        self.0 == self.2
    }

    pub fn is_vertical(&self) -> bool {
        self.1 == self.3
    }
}

impl From<&str> for Line {
    fn from(input: &str) -> Self {
        let mut split = input
            .split(" -> ")
            .flat_map(|s| s.split(","))
            .map(|num| num.parse::<usize>().unwrap());
        Self(
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
        )
    }
}

fn main() {
    let input = std::fs::read_to_string("res/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    count_intersecting_positions(
        input
            .lines()
            .map(|line_str| Line::from(line_str))
            .filter(|line| line.is_aligned())
            .collect::<Vec<_>>(),
    )
}

fn part2(input: &str) -> usize {
    count_intersecting_positions(
        input
            .lines()
            .map(|line_str| Line::from(line_str))
            .collect::<Vec<_>>(),
    )
}

fn count_intersecting_positions(lines: Vec<Line>) -> usize {
    let (max_x, max_y) = lines.iter().fold((0usize, 0usize), |(x, y), line| {
        (x.max(line.0.max(line.2)), y.max(line.1.max(line.3))) // Get the maximum x and y values
    });

    let mut board = vec![vec![0usize; max_x + 1]; max_y + 1];

    for line in lines {
        let mut x = line.0;
        let mut y = line.1;
        // Get the step direction for x and y
        let x_step = (line.2 as isize - line.0 as isize).signum();
        let y_step = (line.3 as isize - line.1 as isize).signum();

        loop {
            board[y][x] += 1;

            if x == line.2 && y == line.3 {
                break;
            }

            x = (x as isize + x_step) as usize;
            y = (y as isize + y_step) as usize;
        }
    }

    board.iter().flatten().filter(|&&v| v >= 2).count()
}
