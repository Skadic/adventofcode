use std::ops::{Index, IndexMut};

use tracing::info;

use crate::{bfs, parse_input, Reachable, Tile, INPUT};

#[derive(Clone, Copy, PartialEq, Eq)]
struct MapTileCounts {
    counts: [OddEvenTileCounts; 9],
}

impl MapTileCounts {
    fn new(tiles: &[Vec<Tile>]) -> MapTileCounts {
        let l = StartPoint::Left.tile_counts(tiles);
        let r = StartPoint::Right.tile_counts(tiles);
        let t = StartPoint::Top.tile_counts(tiles);
        let b = StartPoint::Bottom.tile_counts(tiles);
        let tr = StartPoint::TopRight.tile_counts(tiles);
        let tl = StartPoint::TopLeft.tile_counts(tiles);
        let br = StartPoint::BottomRight.tile_counts(tiles);
        let bl = StartPoint::BottomLeft.tile_counts(tiles);
        let c = StartPoint::Center.tile_counts(tiles);

        MapTileCounts {
            counts: [l, r, t, b, tr, tl, br, bl, c],
        }
    }
}

impl Index<StartPoint> for MapTileCounts {
    type Output = OddEvenTileCounts;

    fn index(&self, index: StartPoint) -> &Self::Output {
        use StartPoint::*;
        match index {
            Left => &self.counts[0],
            Right => &self.counts[1],
            Top => &self.counts[2],
            Bottom => &self.counts[3],
            TopRight => &self.counts[4],
            TopLeft => &self.counts[5],
            BottomRight => &self.counts[6],
            BottomLeft => &self.counts[7],
            Center => &self.counts[8],
        }
    }
}

impl IndexMut<StartPoint> for MapTileCounts {
    fn index_mut(&mut self, index: StartPoint) -> &mut Self::Output {
        use StartPoint::*;
        match index {
            Left => &mut self.counts[0],
            Right => &mut self.counts[1],
            Top => &mut self.counts[2],
            Bottom => &mut self.counts[3],
            TopRight => &mut self.counts[4],
            TopLeft => &mut self.counts[5],
            BottomRight => &mut self.counts[6],
            BottomLeft => &mut self.counts[7],
            Center => &mut self.counts[8],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OddEven {
    Odd,
    Even,
}

impl OddEven {
    fn flip(self) -> Self {
        match self {
            OddEven::Even => OddEven::Odd,
            OddEven::Odd => OddEven::Even,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OddEvenTileCounts {
    odd: TileCounts,
    even: TileCounts,
}

impl Index<OddEven> for OddEvenTileCounts {
    type Output = TileCounts;

    fn index(&self, index: OddEven) -> &Self::Output {
        match index {
            OddEven::Odd => &self.odd,
            OddEven::Even => &self.even,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartPoint {
    Left,
    Right,
    Top,
    Bottom,
    TopRight,
    BottomRight,
    TopLeft,
    BottomLeft,
    Center,
}

impl StartPoint {
    fn tile_counts(self, tiles: &[Vec<Tile>]) -> OddEvenTileCounts {
        let w = tiles[0].len();
        let h = tiles.len();
        use StartPoint::*;
        let start = match self {
            Left => (0, h / 2),
            Right => (w - 1, h / 2),
            Top => (w / 2, 0),
            Bottom => (w / 2, h - 1),
            TopRight => (0, 0),
            TopLeft => (w - 1, 0),
            BottomRight => (w - 1, h - 1),
            BottomLeft => (0, h - 1),
            Center => (w / 2, h / 2),
        };

        let reachable = bfs(tiles, start.0, start.1);

        let xys = (0..h).flat_map(|y| (0..w).zip(std::iter::repeat(y)));

        let mid = w / 2;
        let (top_left_even, top_left_odd) = {
            let mut even = 0;
            let mut odd = 0;
            xys.clone()
                .filter(|&(x, y)| x + y < mid)
                .for_each(|(x, y)| match reachable[y][x] {
                    Reachable::Even => even += 1,
                    Reachable::Odd => odd += 1,
                    _ => {}
                });
            (even, odd)
        };
        let (top_right_even, top_right_odd) = {
            let mut even = 0;
            let mut odd = 0;
            xys.clone()
                .filter(|&(x, y)| (w - x - 1) + y < mid)
                .for_each(|(x, y)| match reachable[y][x] {
                    Reachable::Even => even += 1,
                    Reachable::Odd => odd += 1,
                    _ => {}
                });
            (even, odd)
        };
        let (bottom_right_even, bottom_right_odd) = {
            let mut even = 0;
            let mut odd = 0;
            xys.clone()
                .filter(|&(x, y)| (w - x - 1) + (h - y - 1) < mid)
                .for_each(|(x, y)| match reachable[y][x] {
                    Reachable::Even => even += 1,
                    Reachable::Odd => odd += 1,
                    _ => {}
                });
            (even, odd)
        };
        let (bottom_left_even, bottom_left_odd) = {
            let mut even = 0;
            let mut odd = 0;
            xys.clone()
                .filter(|&(x, y)| x + (h - y - 1) < mid)
                .for_each(|(x, y)| match reachable[y][x] {
                    Reachable::Even => even += 1,
                    Reachable::Odd => odd += 1,
                    _ => {}
                });
            (even, odd)
        };

        let (total_even, total_odd) = {
            let mut even = 0;
            let mut odd = 0;
            xys.for_each(|(x, y)| match reachable[y][x] {
                Reachable::Even => even += 1,
                Reachable::Odd => odd += 1,
                _ => {}
            });
            (even, odd)
        };

        let even = TileCounts::new(
            total_even,
            top_left_even,
            bottom_left_even,
            top_right_even,
            bottom_right_even,
        );
        let odd = TileCounts::new(
            total_odd,
            top_left_odd,
            bottom_left_odd,
            top_right_odd,
            bottom_right_odd,
        );

        OddEvenTileCounts { odd, even }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Segment {
    Full,
    TopRight,
    BottomRight,
    TopLeft,
    BottomLeft,
    Center,
    LeftCorners,
    TopCorners,
    RightCorners,
    BottomCorners,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct TileCounts {
    counts: [usize; 10],
}

impl TileCounts {
    pub fn new(
        full: usize,
        top_left: usize,
        bottom_left: usize,
        top_right: usize,
        bottom_right: usize,
    ) -> TileCounts {
        let mut counts = TileCounts::default();

        counts[Segment::Full] = full;
        counts[Segment::TopRight] = top_right;
        counts[Segment::BottomRight] = bottom_right;
        counts[Segment::TopLeft] = top_left;
        counts[Segment::BottomLeft] = bottom_left;
        counts[Segment::Center] = full - top_left - bottom_left - top_right - bottom_right;
        counts[Segment::LeftCorners] = top_left + bottom_left;
        counts[Segment::RightCorners] = top_right + bottom_right;
        counts[Segment::TopCorners] = top_left + top_right;
        counts[Segment::BottomCorners] = bottom_right + bottom_left;

        counts
    }
}

impl Index<Segment> for TileCounts {
    type Output = usize;

    fn index(&self, index: Segment) -> &Self::Output {
        match index {
            Segment::Full => &self.counts[0],
            Segment::TopRight => &self.counts[1],
            Segment::TopLeft => &self.counts[2],
            Segment::BottomLeft => &self.counts[3],
            Segment::BottomRight => &self.counts[4],
            Segment::Center => &self.counts[5],
            Segment::LeftCorners => &self.counts[6],
            Segment::TopCorners => &self.counts[7],
            Segment::RightCorners => &self.counts[8],
            Segment::BottomCorners => &self.counts[9],
        }
    }
}

impl IndexMut<Segment> for TileCounts {
    fn index_mut(&mut self, index: Segment) -> &mut Self::Output {
        match index {
            Segment::Full => &mut self.counts[0],
            Segment::TopRight => &mut self.counts[1],
            Segment::TopLeft => &mut self.counts[2],
            Segment::BottomLeft => &mut self.counts[3],
            Segment::BottomRight => &mut self.counts[4],
            Segment::Center => &mut self.counts[5],
            Segment::LeftCorners => &mut self.counts[6],
            Segment::TopCorners => &mut self.counts[7],
            Segment::RightCorners => &mut self.counts[8],
            Segment::BottomCorners => &mut self.counts[9],
        }
    }
}

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let (map, _, _) = parse_input(INPUT);

    let counts = MapTileCounts::new(&map)[StartPoint::Center];
    let mut orientation = OddEven::Odd;

    const STEP_SIZE: usize = 26501365;
    //info!(res = (STEP_SIZE - 65) % 131);
    //const STEP_SIZE: usize = 10;
    let w = map[0].len();

    let range_included = STEP_SIZE / w - 1;
    info!(range_included);
    let mut count = counts[orientation][Segment::Full];
    for i in 1..=range_included {
        orientation = orientation.flip();
        count += i * 4 * counts[orientation][Segment::Full];
    }
    orientation = orientation.flip();
    info!(inside = count);

    let count_now = count;
    count += counts[orientation][Segment::Full] - counts[orientation][Segment::LeftCorners];
    count += counts[orientation][Segment::Full] - counts[orientation][Segment::TopCorners];
    count += counts[orientation][Segment::Full] - counts[orientation][Segment::RightCorners];
    count += counts[orientation][Segment::Full] - counts[orientation][Segment::BottomCorners];
    info!(
        lc = counts[orientation][Segment::LeftCorners],
        rc = counts[orientation][Segment::RightCorners]
    );
    info!(
        tc = counts[orientation][Segment::TopCorners],
        bc = counts[orientation][Segment::BottomCorners]
    );
    info!(corners = count - count_now);

    let count_now = count;
    count += (counts[orientation][Segment::Full] - counts[orientation][Segment::TopLeft])
        * range_included;
    count += (counts[orientation][Segment::Full] - counts[orientation][Segment::TopRight])
        * range_included;
    count += (counts[orientation][Segment::Full] - counts[orientation][Segment::BottomLeft])
        * range_included;
    count += (counts[orientation][Segment::Full] - counts[orientation][Segment::BottomRight])
        * range_included;
    info!(half_in = count - count_now);

    // Outside
    let count_now = count;
    orientation = orientation.flip();
    count += counts[orientation][Segment::BottomRight] * (range_included + 1);
    count += counts[orientation][Segment::TopLeft] * (range_included + 1);
    count += counts[orientation][Segment::TopRight] * (range_included + 1);
    count += counts[orientation][Segment::BottomLeft] * (range_included + 1);
    count -= range_included + 1;
    info!(half_out = count - count_now);
    info!(result = count);
    assert_eq!(605492675373144, count, "diff: {}", count.abs_diff(605492675373144));

    info!(w = map[0].len(), h = map.len());

    /*
    for (row, l) in bfs(&map, 1, 1).iter().enumerate() {
        let b = l
            .into_iter()
            .map(|&b| match b {
                Reachable::Never => '#',
                Reachable::Even => '1',
                Reachable::Odd => '0',
            })
            .collect::<String>();
        info!(row, "{b}")
    }
    */
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
