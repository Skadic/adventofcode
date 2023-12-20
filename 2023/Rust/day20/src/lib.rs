use std::{
    cell::RefCell,
    collections::HashMap,
    ops::Not,
};

use miette::IntoDiagnostic;
use nom::Parser;
use nom_supreme::ParserExt;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
pub const SAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

mod parser;
pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Pulse {
    High,
    #[default]
    Low,
}

impl Pulse {
    pub fn is_high(self) -> bool {
        matches!(self, Pulse::High)
    }

    pub fn is_low(self) -> bool {
        matches!(self, Pulse::Low)
    }

    pub fn flip(self) -> Self {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}
impl Not for Pulse {
    type Output = Pulse;

    fn not(self) -> Self::Output {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Module {
    name: &'static str,
    tp: ModuleType,
    inputs: Vec<&'static str>,
    outputs: Vec<&'static str>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ModuleType {
    FlipFlop(RefCell<Pulse>),
    Conjunction(RefCell<HashMap<&'static str, Pulse>>),
    Broadcast,
}

pub static mut LOW_COUNT: usize = 0;
pub static mut HIGH_COUNT: usize = 0;
pub static mut RX_COUNT: usize = 0;

impl Module {
    pub fn add_output(&mut self, new: &'static str) {
        self.outputs.push(new);
    }

    pub fn add_input(&mut self, new: &'static str) {
        self.inputs.push(new);
    }

    pub fn inputs(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.inputs.iter().copied()
    }

    pub fn outputs(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.outputs.iter().copied()
    }

    pub fn receive(
        &self,
        src: &'static str,
        pulse: Pulse,
        modules: &HashMap<&'static str, Module>,
    ) {
        match self.tp {
            ModuleType::Broadcast => self.send(pulse, modules),
            ModuleType::FlipFlop(_) if pulse.is_high() => {}
            ModuleType::FlipFlop(ref p) => {
                let new_val = p.borrow().flip();
                *p.borrow_mut() = new_val;
                self.send(new_val, modules);
            }
            ModuleType::Conjunction(ref memory) => {
                memory.borrow_mut().insert(src, pulse);
                let borrowed_mem = memory.borrow();
                let should_send_low = self
                    .inputs
                    .iter()
                    .all(|input| borrowed_mem.get(input).unwrap_or(&Pulse::Low).is_high());
                drop(borrowed_mem);
                let send_val = if should_send_low {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                self.send(send_val, modules);
            }
        }
    }

    pub fn send(&self, pulse: Pulse, modules: &HashMap<&'static str, Module>) {
        for &v in self.outputs.iter() {
            unsafe {
                match pulse {
                    Pulse::High => HIGH_COUNT += 1,
                    Pulse::Low => LOW_COUNT += 1,
                }
            }
            let module = modules.get(v).unwrap();
            module.receive(self.name, pulse, modules)
        }
    }

    pub fn propagate(&self, in_pulse: Pulse, modules: &HashMap<&'static str, Module>) {
        let mut queue = Vec::new();
        queue.extend(self.outputs().map(|s| (self.name, in_pulse, s)));
        while !queue.is_empty() {
            let mut to_send = Vec::new();
            for (src, pulse, target) in std::mem::take(&mut queue) {
                unsafe {
                    match pulse {
                        Pulse::High => HIGH_COUNT += 1,
                        Pulse::Low => LOW_COUNT += 1,
                    }
                }

                let module = &modules[target];
                if module.name == "rx" && pulse == Pulse::Low {
                    unsafe {
                        RX_COUNT += 1;
                    }
                }

                match module.tp {
                    ModuleType::Broadcast => {
                        to_send.push((Some(pulse), module.name));
                    }
                    ModuleType::FlipFlop(_) if pulse.is_high() => {}
                    ModuleType::FlipFlop(ref p) => {
                        let new_val = p.borrow().flip();
                        *p.borrow_mut() = new_val;
                        to_send.push((None, module.name));
                    }
                    ModuleType::Conjunction(ref memory) => {
                        memory.borrow_mut().insert(src, pulse);
                        to_send.push((None, module.name));
                    }
                }
            }

            for (pulse, target) in to_send.into_iter() {
                let module = &modules[target];
                match module.tp {
                    ModuleType::Broadcast => {
                        for o in module.outputs() {
                            queue.push((target, pulse.unwrap(), o));
                        }
                    }
                    ModuleType::FlipFlop(ref p) => {
                        for o in module.outputs() {
                            queue.push((target, *p.borrow(), o));
                        }
                    }
                    ModuleType::Conjunction(ref memory) => {
                        let borrowed_mem = memory.borrow();
                        let should_send_low = module
                            .inputs
                            .iter()
                            .all(|input| borrowed_mem.get(input).unwrap_or(&Pulse::Low).is_high());
                        drop(borrowed_mem);
                        let send_val = if should_send_low {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };

                        for o in module.outputs() {
                            queue.push((target, send_val, o));
                        }
                    }
                }
            }
        }
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &'static str) -> miette::Result<HashMap<&'static str, Module>> {
    parser::modules
        .all_consuming()
        .parse(input.trim())
        .map(|(_, v)| v)
        .into_diagnostic()
}
