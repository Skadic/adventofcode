use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::{
        complete::{char, digit1, line_ending, newline},
        streaming::{multispace0, multispace1},
    },
    combinator::{map, map_res, opt, peek},
    multi::many1,
    sequence::{delimited, pair, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub num_boxes: usize,
    pub source: usize,
    pub target: usize,
}

fn box_parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Vec<Option<char>>>> {
    // Parse a single box
    let box_parser = delimited(char('['), take(1usize), char(']'));
    // Parse an empty space where there is no box
    let no_box_parser = map(tag("   "), str::trim);
    // Parse either a box or an empty space, and recognize a newline or discard the following space
    // We don't discard the newline since we need to know when this row ends
    let any_box_parser = terminated(
        alt((box_parser, no_box_parser)),
        alt((char(' '), peek(newline))),
    );
    // Create a char from the box' value, if there is no box "None" will be in its place
    let char_box_parser = map(any_box_parser, |val| str::parse::<char>(val).ok());
    // Parse a row of boxes
    let box_row_parser = terminated(many1(char_box_parser), line_ending);
    // Parse all rows of boxes
    let box_parser = many1(box_row_parser);

    box_parser
}

fn instruction_parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Instruction>> {
    // Parse a single instruction line
    let single_instruction_parser = tuple((
        terminated(tag("move"), multispace1),
        terminated(map_res(digit1, str::parse::<usize>), multispace1),
        terminated(tag("from"), multispace1),
        terminated(map_res(digit1, str::parse::<usize>), multispace1),
        terminated(tag("to"), multispace1),
        map_res(digit1, str::parse::<usize>),
    ));

    let single_instruction_parser = terminated(single_instruction_parser, opt(newline));

    let single_instruction_parser = map(
        single_instruction_parser,
        |(_, num_boxes, _, source, _, target)| Instruction {
            num_boxes,
            source: source - 1,
            target: target - 1,
        },
    );
    many1(single_instruction_parser)
}

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let box_parser = box_parser();
    let instruction_parser = instruction_parser();
    let mut parser = separated_pair(box_parser, take_until("move"), instruction_parser);

    let (boxes_row_wise, instructions) = parser(input).unwrap().1;
    let stack_count = boxes_row_wise.iter().map(Vec::len).max().unwrap();
    let max_boxes = boxes_row_wise.len();

    let boxes = (0..stack_count)
        .map(|stack_id| {
            (0..max_boxes).rev()
                .filter_map(|i| boxes_row_wise[i][stack_id])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (boxes, instructions)
}

#[cfg(test)]
mod test {
    use nom::{bytes::complete::take_until, sequence::separated_pair};

    use crate::{box_parser, instruction_parser};

    #[test]
    fn test_parsing() {
        let input = include_str!("../input.txt");
        let box_parser = box_parser();
        let instruction_parser = instruction_parser();
        let mut parser = separated_pair(box_parser, take_until("move"), instruction_parser);

        assert!(parser(input).is_ok(), "Parsing unsuccessful")
    }
}
