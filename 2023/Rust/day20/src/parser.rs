use std::collections::HashMap;

use crate::{Module, ModuleType};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{self as chars, multispace1, space0},
    multi::separated_list1,
    sequence::{self, delimited, preceded},
    IResult, Parser,
};
use nom_supreme::ParserExt;

pub fn module(input: &'static str) -> IResult<&'static str, Module> {
    enum ModTp {
        Broadcast,
        Conjunction(&'static str),
        FlipFlop(&'static str),
    }

    let broadcast = tag("broadcaster").map(|_: &'static str| ModTp::Broadcast);
    let conj = preceded(
        chars::char('&'),
        take_while1(|c: char| c.is_ascii_lowercase()).cut(),
    )
    .map(ModTp::Conjunction);
    let flipflop = preceded(
        chars::char('%'),
        take_while1(|c: char| c.is_ascii_lowercase()).cut(),
    )
    .map(ModTp::FlipFlop);

    let (next, mod_type) = alt((broadcast, conj, flipflop))(input)?;

    let (name, module) = match mod_type {
        ModTp::Broadcast => ("broadcaster", ModuleType::Broadcast),
        ModTp::Conjunction(s) => (s, ModuleType::Conjunction(Default::default())),
        ModTp::FlipFlop(s) => (s, ModuleType::FlipFlop(Default::default())),
    };

    let (next, _) = delimited(space0, tag("->"), space0)(next)?;

    let (next, outputs) = separated_list1(
        sequence::pair(chars::char(','), space0),
        take_while1(|c: char| c.is_ascii_lowercase()),
    )(next)?;

    let m = Module {
        name,
        tp: module,
        inputs: vec![],
        outputs,
    };

    Ok((next, m))
}

pub fn modules(input: &'static str) -> IResult<&'static str, HashMap<&'static str, Module>> {
    let (next, module_vec) = separated_list1(multispace1, module)(input)?;
    let name_vec = module_vec.iter().map(|v| v.name).collect::<Vec<_>>();

    let mut modules = module_vec
        .into_iter()
        .map(|m| (m.name, m))
        .collect::<HashMap<_, _>>();

    for module in name_vec.into_iter() {
        let o = modules[module].outputs().collect::<Vec<_>>();
        for output in o {
            modules
                .entry(output)
                .or_insert_with(|| Module {
                    name: output,
                    inputs: vec![],
                    outputs: vec![],
                    tp: ModuleType::Broadcast,
                })
                .add_input(module);
        }
    }

    Ok((next, modules))
}
