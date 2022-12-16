use std::{fmt::Display, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::streaming::take_while,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::delimited,
    AsChar, IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PacketElem {
    Packet(usize),
    List(Vec<PacketElem>),
}

impl Display for PacketElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketElem::Packet(v) => write!(f, "{v}"),
            PacketElem::List(elems) => write!(
                f,
                "[{}]",
                { let s = elems
                    .iter()
                    .map(PacketElem::to_string)
                    .map(|elem| format!("{elem},"))
                    .collect::<String>();

                    if s.is_empty() {
                        "".to_string()
                    } else {
                        s[..s.len()-1].to_string()
                    }
                }
            ),
        }
    }
}

impl PartialOrd for PacketElem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use PacketElem::*;
        Some(match (self, other) {
            (Packet(mine), Packet(others)) => mine.cmp(others),
            (Packet(mine), List(others)) => [Packet(*mine)].as_slice().cmp(&others[..]),
            (List(mine), Packet(others)) => mine.as_slice().cmp([Packet(*others)].as_slice()),
            (List(mine), List(others)) => mine.cmp(others),
        })
    }
}

impl Ord for PacketElem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn number(s: &str) -> IResult<&str, PacketElem> {
    map(
        map_res(
            take_while(<char as AsChar>::is_dec_digit),
            str::parse::<usize>,
        ),
        PacketElem::Packet,
    )(s)
}

fn list_elem(s: &str) -> IResult<&str, PacketElem> {
    alt((
        number,
        map(
            delimited(
                tag("["),
                separated_list0(tag(","), alt((number, list_elem))),
                tag("]"),
            ),
            PacketElem::List,
        ),
    ))(s)
}

impl FromStr for PacketElem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        list_elem(s)
            .map(|(_, vec)| vec)
            .map_err(|e| format!("error parsing packet element: {e}"))
    }
}

pub fn process_input(input: &str) -> Vec<(PacketElem, PacketElem)> {
    let mut packets = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::parse::<PacketElem>)
        .collect::<Result<Vec<_>, _>>()
        .expect("error parsing elements");

    packets
        .chunks_exact_mut(2)
        .map(|elems| {
            (
                std::mem::replace(&mut elems[0], PacketElem::Packet(0)),
                std::mem::replace(&mut elems[1], PacketElem::Packet(0)),
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::process_input;

    #[test]
    fn test_part1() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let packets = process_input(input);
        let indices = packets
            .into_iter()
            .enumerate()
            .filter_map(|(i, (l, r))| if l < r { Some(i + 1) } else { None })
            .collect::<Vec<_>>();
        println!("{:?}", &indices);
        assert_eq!(13, indices.into_iter().sum::<usize>());
    }
}
