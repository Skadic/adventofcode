pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
pub const TERMINATOR: char = '\n';

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Grid {
    data: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

impl Grid {
    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }
}

impl<YIter, XIter> From<YIter> for Grid
where
    YIter: IntoIterator<Item = XIter>,
    XIter: IntoIterator<Item = char>,
{
    fn from(value: YIter) -> Self {
        let data: Vec<Vec<char>> = value
            .into_iter()
            .map(IntoIterator::into_iter)
            .map(Iterator::collect)
            .collect::<Vec<_>>();
        let w = data[0].len();
        let h = data.len();
        Self { data, w, h }
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Grid {
    input.lines().map(str::chars).into()
}
