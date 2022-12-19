type Position = (isize, isize);

pub mod copy_grid;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, *,
};

fn position_parser(token: &str) -> IResult<&str, Position> {
    separated_pair(
        preceded(tag("x="), character::complete::i64),
        tag(", "),
        preceded(tag("y="), character::complete::i64),
    )
    .map(|(x, y)| (x as isize, y as isize))
    .parse(token)
}

fn sensor_parser(token: &str) -> IResult<&str, (Position, Position)> {
    let (token, sensor_pos) = delimited(
        terminated(tag("Sensor at"), multispace0),
        position_parser,
        terminated(tag(":"), multispace0),
    )(token)?;
    let (token, beacon_pos) = preceded(
        terminated(tag("closest beacon is at"), multispace0),
        position_parser,
    )(token)?;

    Ok((token, (sensor_pos, beacon_pos)))
}

fn all_sensors_parser(token: &str) -> IResult<&str, Vec<(Position, Position)>> {
    separated_list0(multispace1, sensor_parser)(token)
}

#[inline]
pub const fn manhattan_distance((x1, y1): Position, (x2, y2): Position) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

pub fn process_part1(input: &str, row: usize) -> usize {
    let row = row as isize;
    let (_, mut data) = all_sensors_parser(input).unwrap();
    data.sort_unstable_by_key(|&((x, _), _)| x);

    let mut count = 0;
    // We need to take care to not count positions twice.
    // This variable represents the x position before which positions should not be counted
    let mut cutoff = isize::MIN;
    for (sensor, beacon) in data {
        let distance = manhattan_distance(sensor, beacon);
        let dist_to_row = sensor.1.abs_diff(row) as isize;
        if dist_to_row < 0 {
            continue;
        }
        let remainder = distance as isize - dist_to_row;
        let min_x = sensor.0 - remainder as isize;
        let max_x = sensor.0 + remainder as isize;

        if cutoff < max_x {
            let start = cutoff.clamp(min_x, max_x + 1);
            let end = max_x;
            count += end - start;
        }
        cutoff = cutoff.max(max_x as isize);
    }

    count as usize
}

pub fn is_valid(pos: Position, others: impl IntoIterator<Item = (Position, Position)>) -> bool {
    others.into_iter().all(|(sensor, beacon)| {
        manhattan_distance(sensor, pos) > manhattan_distance(sensor, beacon)
    })
}

pub fn process_part2(input: &str) -> usize {
    let (_, mut data) = all_sensors_parser(input).unwrap();
    data.sort_unstable_by_key(|&((x, _), _)| x);

    let mut v = vec![];

    for &(sensor @ (sx, sy), beacon) in data.iter() {
        let dist = manhattan_distance(sensor, beacon) as isize + 1;
        for i in 0..=dist {
            [
                (sx + i, sy - dist + i),
                (sx - i, sy - dist + i),
                (sx + i, sy + dist - i),
                (sx - i, sy + dist - i),
            ]
            .into_iter()
            .filter(|&p| is_valid(p, data.iter().copied()))
            .filter(|&(x, y)| x == x.clamp(0, 4000000) && y == y.clamp(0, 4000000))
            .for_each(|pos| v.push(pos));
        }
    }

    println!("{v:?}");
    (v[0].0 * 4000000 + v[0].1) as usize
}

#[cfg(test)]
mod test {
    use crate::process_part1;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(26, process_part1(INPUT, 10));
    }
}
