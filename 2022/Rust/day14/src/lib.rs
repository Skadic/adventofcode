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

    #[inline]
    fn new_copied(width: usize, height: usize, t: T) -> Self {
        Self {
            data: (0..width * height).map(|_| t).collect(),
            width,
            height,
        }
    }
}

#[allow(unused)]
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
    pub fn with_data(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, v: T) -> T {
        let w = self.width();
        std::mem::replace(&mut self.data[y * w + x], v)
    }

    #[inline]
    pub fn get_ref(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapElem {
    Air,
    Rock,
    Sand,
    SandSource,
}

impl MapElem {
    pub fn is_solid(self) -> bool {
        self.is_rock() || self.is_sand()
    }

    pub fn is_rock(self) -> bool {
        matches!(self, MapElem::Rock)
    }

    pub fn is_sand(self) -> bool {
        matches!(self, MapElem::Sand)
    }
}

pub fn print_map(map: &CopyGrid<MapElem>) {
    for ((x, _), element) in map.entries() {
        if x < 400 {
            continue;
        }
        use MapElem::*;
        print!(
            "{}",
            match element {
                Air => '.',
                Rock => '#',
                Sand => 'o',
                SandSource => '+',
            }
        );

        if x == map.width() - 1 {
            println!()
        }
    }
}

pub fn process_input(input: &str) -> CopyGrid<MapElem> {
    let paths = input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.split(" -> ")
                .map(|path_pos| {
                    let mut split = path_pos.split(',');
                    let x = split.next().unwrap().parse::<usize>().unwrap();
                    let y = split.next().unwrap().parse::<usize>().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (max_x, max_y) = paths.iter().flat_map(|path| path.iter()).fold(
        (usize::MIN, usize::MIN),
        |(old_max_x, old_max_y), &(x, y)| (old_max_x.max(x), old_max_y.max(y)),
    );

    let mut map = CopyGrid::<MapElem>::new_copied((max_x + 1) * 2, max_y + 3, MapElem::Air);

    for path in paths {
        for (&(x1, y1), &(x2, y2)) in path.iter().zip(path.iter().skip(1)) {
            if x1 == x2 {
                for (x, y) in
                    std::iter::repeat(x1 as usize).zip(y1.min(y2) as usize..=y1.max(y2) as usize)
                {
                    map.set(x, y, MapElem::Rock);
                }
            } else {
                for (x, y) in
                    (x1.min(x2) as usize..=x1.max(x2) as usize).zip(std::iter::repeat(y1 as usize))
                {
                    map.set(x, y, MapElem::Rock);
                }
            }
        }
    }

    map.set(500, 0, MapElem::SandSource);

    print_map(&map);

    map
}
