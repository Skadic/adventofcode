use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take, take_while, take_while1},
    character::complete::{multispace0, space0, space1},
    combinator::{map, map_res},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

fn is_digit(c: char) -> bool {
    let c = c as u8;
    '0' as u8 <= c && c <= '9' as u8
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operator::*;
        Ok(match s {
            "+" => Add,
            "*" => Mul,
            _ => return Err(format!("invalid operation: {s}")),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Binary(Operator),
    Left(usize, Operator),
    Right(Operator, usize),
}

impl Operation {}

fn num_parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, usize> {
    map_res(take_while1(is_digit), usize::from_str)
}

fn monkey_parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Monkey>> {
    let header_parser = delimited(tuple((tag("Monkey"), space1)), num_parser(), tag(":"));
    let starting_items_parse = preceded(
        pair(tag("Starting items:"), space0),
        separated_list1(pair(tag(","), space0), num_parser()),
    );

    let operation_parser = map(
        tuple((
            terminated(take_while(char::is_alphanumeric), space1),
            terminated(map_res(take(1usize), Operator::from_str), space1),
            take_while(char::is_alphanumeric),
        )),
        |(lhs, op, rhs)| {
            if lhs == rhs && lhs == "old" {
                return Operation::Binary(op);
            }
            if let Ok(lhs) = lhs.parse::<usize>() {
                return Operation::Left(lhs, op);
            }
            return Operation::Right(op, rhs.parse::<usize>().unwrap());
        },
    );

    let operation_line_parser = preceded(
        tuple((
            tag("Operation:"),
            space0,
            tag("new"),
            space0,
            tag("="),
            space0,
        )),
        operation_parser,
    );

    let test_parser = preceded(
        tuple((
            tag("Test:"),
            space0,
            tag("divisible"),
            space1,
            tag("by"),
            space1,
        )),
        num_parser(),
    );
    let true_target_path_parser = preceded(
        tuple((
            tag("If"),
            space1,
            tag("true"),
            space0,
            tag(":"),
            space0,
            tag("throw"),
            space1,
            tag("to"),
            space1,
            tag("monkey"),
            space1,
        )),
        num_parser(),
    );
    let false_target_path_parser = preceded(
        tuple((
            tag("If"),
            space1,
            tag("false"),
            space0,
            tag(":"),
            space0,
            tag("throw"),
            space1,
            tag("to"),
            space1,
            tag("monkey"),
            space1,
        )),
        num_parser(),
    );

    let monkey_parser = tuple((
        terminated(header_parser, multispace0),
        terminated(starting_items_parse, multispace0),
        terminated(operation_line_parser, multispace0),
        terminated(test_parser, multispace0),
        terminated(true_target_path_parser, multispace0),
        terminated(false_target_path_parser, multispace0),
    ));

    let monkey_parser = map(
        monkey_parser,
        |(id, items, operation, div_test_value, true_target_monkey, false_target_monkey)| Monkey {
            id,
            items,
            operation,
            div_test_value,
            true_target_monkey,
            false_target_monkey,
        },
    );

    many0(monkey_parser)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<usize>,
    pub operation: Operation,
    pub div_test_value: usize,
    pub true_target_monkey: usize,
    pub false_target_monkey: usize,
}

impl Monkey {
    pub fn target_monkey(&self, item: usize) -> usize {
        match item % self.div_test_value {
            0 => self.true_target_monkey,
            _ => self.false_target_monkey,
        }
    }

    pub fn eval(&self, item: usize) -> usize {
        use Operator::*;
        match self.operation {
            Operation::Binary(op) => match op {
                Add => item << 2,
                Mul => item * item,
            },
            Operation::Left(val, op) => match op {
                Add => val + item,
                Mul => val * item,
            },
            Operation::Right(op, val) => match op {
                Add => item + val,
                Mul => item * val,
            },
        }
    }
}

fn process_input(input: &str) -> Vec<Monkey> {
    monkey_parser()(input).unwrap().1
}

pub fn process_part1(input: &str) -> usize {
    let mut monkeys = process_input(input);
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items_thrown = std::mem::take(&mut monkeys[i].items);
            inspections[i] += items_thrown.len();
            for item in items_thrown {
                let item = monkeys[i].eval(item) / 3;
                // Throw item to target monkey
                let target = monkeys[i].target_monkey(item);
                monkeys[target].items.push(item);
            }
        }
    }

    inspections.sort();

    inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
}

pub fn process_part2(input: &str) -> usize {
    let mut monkeys = process_input(input);
    let mut inspections = vec![0; monkeys.len()];

    // Find the mod value of the ring for which arithmetic is the same for all monkeys
    // That is just the product of all values the monkeys use to check the item
    let div_value: usize = monkeys.iter().map(|monkey| monkey.div_test_value).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items_thrown = std::mem::take(&mut monkeys[i].items);
            inspections[i] += items_thrown.len();
            for item in items_thrown {
                let item = monkeys[i].eval(item) % div_value;
                // Throw item to target monkey
                let target = monkeys[i].target_monkey(item);
                monkeys[target].items.push(item);
            }
        }
    }
    println!("{inspections:?}");

    inspections.sort();

    inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
}

#[cfg(test)]
mod test {
    use crate::process_input;

    #[test]
    fn test_parser() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let output = process_input(input);
        assert_eq!(4, output.len())
    }
}
