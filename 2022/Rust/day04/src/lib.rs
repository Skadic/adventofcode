use nom::{
    character::{complete::char},
    combinator::map_res,
    sequence::separated_pair,
    IResult, bytes::complete::{take_while, tag},
};

pub fn includes_second<T: PartialOrd>(
    (first_l, first_r): (T, T),
    (second_l, second_r): (T, T),
) -> bool {
    first_l <= second_l && second_r <= first_r
}

pub fn overlaps<T: PartialOrd + Copy>(
    first@(first_l, first_r): (T, T),
    second@(second_l, _): (T, T),
) -> bool {
    if first_l > second_l {
        overlaps(second, first)
    } else {
        second_l <= first_r 
    }
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> IResult<&str, Vec<((usize, usize), (usize, usize))>> {
    let mut range = separated_pair(
        map_res(take_while(|c: char| c.is_ascii_digit()), str::parse::<usize>),
        tag("-"),
        map_res(take_while(|c: char| c.is_ascii_digit()), str::parse::<usize>),
    );

    let mut v = vec![];
    for line in input.lines().map(str::trim) {
        let (rem, first) = range(line)?;
        let (rem, _) = char(',')(rem)?;
        let (_, second) = range(rem)?;
        v.push((first, second));
    }

    Ok(("", v))
}

pub fn process_input(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    parse_input(input).unwrap().1
}
