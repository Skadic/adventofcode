use std::cmp::Ordering;
use std::collections::HashMap;

use miette::{miette, Context, IntoDiagnostic};
use nom::character::complete::multispace1;
use nom::Finish;
use nom::{multi::separated_list1, Parser};
use nom_supreme::ParserExt;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

pub mod part1;
pub mod part2;

mod parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Field {
    type Error = miette::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => return Err(miette!("invalid char: {value}")),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    Compare(Field, Ordering, usize),
    Pass,
}

impl Condition {
    pub fn matches(&self, part: &Part) -> bool {
        match *self {
            Self::Pass => true,
            Self::Compare(field, ordering, ref int) => part.get(field).cmp(int) == ordering,
        }
    }

    pub fn constrain(&self, part: PartRange) -> Option<PartRange> {
        match self {
            _ if !part.compatible(*self) => None,
            Self::Pass => Some(part.clone()),
            Self::Compare(field, Ordering::Greater, int) => {
                let mut part = part.clone();
                part.get_mut(*field).constrain_min(int + 1);
                Some(part)
            }
            Self::Compare(field, Ordering::Less, int) => {
                let mut part = part.clone();
                part.get_mut(*field).constrain_max(int - 1);
                Some(part)
            }
            _ => Some(part.clone()),
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Self::Compare(field, Ordering::Greater, int) => {
                Self::Compare(field, Ordering::Less, int + 1)
            }
            Self::Compare(field, Ordering::Less, int) => {
                Self::Compare(field, Ordering::Greater, int - 1)
            }
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Workflow(&'static str),
    Accept,
    Reject,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct NumRange {
    /// inclusive
    min: u16,
    /// inclusive
    max: u16,
}

impl NumRange {
    pub fn constrain_max(&mut self, new_max: usize) {
        self.max = self.max.min(new_max as u16);
    }

    pub fn constrain_min(&mut self, new_min: usize) {
        self.min = self.min.max(new_min as u16);
    }

    pub fn count(&self) -> usize {
        self.max as usize - self.min as usize + 1
    }
}

#[derive(Debug)]
struct Workflow {
    actions: Vec<(Condition, Target)>,
}

impl Workflow {
    pub fn target(&self, part: &Part) -> Target {
        self.actions
            .iter()
            .find(|(cond, _)| cond.matches(part))
            .unwrap()
            .1
    }

    pub fn with_range<'a>(
        &'a self,
        part: PartRange,
    ) -> impl Iterator<Item = (PartRange, Target)> + 'a {
        self.actions
            .iter()
            .copied()
            .scan(part, move |p, (cond, target)| {
                let Some(constrained) = cond.constrain(*p) else {
                    return Some(None);
                };

                if let Some(r) = cond.flip().constrain(*p) {
                    *p = r;
                }

                Some(Some((constrained, target)))
            })
            .filter_map(|opt| opt)
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: NumRange,
    m: NumRange,
    a: NumRange,
    s: NumRange,
}

impl Part {
    pub fn get(&self, field: Field) -> usize {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }

    pub fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl PartRange {
    pub fn get(&self, field: Field) -> &NumRange {
        match field {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }

    pub fn get_mut(&mut self, field: Field) -> &mut NumRange {
        match field {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }
    }

    pub fn compatible(&self, cond: Condition) -> bool {
        match cond {
            Condition::Compare(field, Ordering::Greater, int) => self.get(field).max > int as u16,
            Condition::Compare(field, Ordering::Less, int) => self.get(field).min < int as u16,
            _ => true,
        }
    }

    pub fn possibilities(&self) -> usize {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }
}

impl Default for PartRange {
    fn default() -> Self {
        Self {
            x: NumRange { min: 1, max: 4000 },
            m: NumRange { min: 1, max: 4000 },
            a: NumRange { min: 1, max: 4000 },
            s: NumRange { min: 1, max: 4000 },
        }
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(
    input: &'static str,
) -> miette::Result<(HashMap<&'static str, Workflow>, Vec<Part>)> {
    let (workflow_str, part_str) = input.split_once("\n\n").unwrap();

    let (_, workflows) = separated_list1(multispace1, parser::workflow)
        .all_consuming()
        .parse(workflow_str.trim())
        .finish()
        .into_diagnostic()
        .wrap_err("error parsing workflows")?;
    let (_, parts) = separated_list1(multispace1, parser::part)
        .all_consuming()
        .parse(part_str.trim())
        .finish()
        .into_diagnostic()
        .wrap_err("error parsing parts")?;

    Ok((workflows.into_iter().collect(), parts))
}
