use std::cmp::Ordering;

use crate::{Condition, Field, Part, Target, Workflow};
use nom::bytes::complete::take_while1;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::{IResult, Parser};
use nom_supreme::ParserExt;

use nom::branch;
use nom::character::complete as chars;

pub fn field(text: &'static str) -> IResult<&str, Field> {
    chars::one_of("xmas")
        .map_res(|c| Field::try_from(c))
        .parse(text)
}

pub fn condition(text: &'static str) -> IResult<&str, Condition> {
    let (next, field) = field(text)?;
    let (next, cond) = chars::one_of("<>").parse(next)?;
    let (next, int) = chars::u64.map(|v| v as usize).parse(next)?;

    let ordering = match cond {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        // This shouldn't be happening
        _ => Ordering::Equal
    };
    Ok((next, Condition::Compare(field, ordering, int)))
}

pub fn target(text: &'static str) -> IResult<&str, Target> {
    let accept = chars::char('A').map(|_| Target::Accept);
    let reject = chars::char('R').map(|_| Target::Reject);
    branch::alt((
        accept,
        reject,
        take_while1(|c: char| c.is_ascii_lowercase()).map(|s| Target::Workflow(s)),
    ))(text)
}

pub fn action(text: &'static str) -> IResult<&str, (Condition, Target)> {
    branch::alt((
        separated_pair(condition, chars::char(':'), target),
        target.map(|t| (Condition::Pass, t)),
    ))
    .parse(text)
}

pub fn workflow(text: &'static str) -> IResult<&str, (&str, Workflow)> {
    let (next, name) = take_while1(|c: char| c.is_ascii_lowercase()).parse(text)?;
    let (next, actions) = delimited(
        chars::char('{'),
        separated_list1(chars::char(','), action),
        chars::char('}'),
    )
    .parse(next)?;

    Ok((next, (name, Workflow { actions })))
}

pub fn part(text: &'static str) -> IResult<&str, Part> {
    let assignment = field
        .precedes(chars::char('='))
        .precedes(chars::u64.map(|v| v as usize));

    let (next, fields): (_, [usize; 4]) = delimited(
        chars::char('{'),
        assignment.separated_array(chars::char(',')),
        chars::char('}'),
    )
    .parse(text)?;


    Ok((
        next,
        Part {
            x: fields[0],
            m: fields[1],
            a: fields[2],
            s: fields[3],
        },
    ))
}
