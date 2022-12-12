use std::{cell::Cell, str::FromStr};

pub struct Cpu {
    instructions: Vec<Instruction>,
    remaining_cycles: usize,
    current_cycle: usize,
    data: Option<isize>,
    x_register: Cell<isize>,
}

impl Cpu {
    pub fn new(instructions: impl IntoIterator<Item = Instruction>) -> Self {
        let mut cpu = Self {
            instructions: instructions.into_iter().collect(),
            remaining_cycles: 0,
            current_cycle: 0,
            data: None,
            x_register: Cell::new(1),
        };
        cpu.instructions.reverse();
        if let Some(last) = cpu.instructions.last() {
            cpu.remaining_cycles = last.cycles;
            cpu.data = last.data;
        }
        cpu
    }

    pub fn clock(&mut self) -> bool {
        self.current_cycle += 1;
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
        }

        // the instruction is still in progress
        if self.remaining_cycles > 0 {
            return true;
        }

        // Get the instruction and run it
        let Some(instruction) = self.instructions.pop() else {
            return false;
        };
        (instruction.run)(self);

        // Prepare for the next instruction
        let Some(next_instruction) = self.instructions.last() else {
            return false;
        };

        self.remaining_cycles = next_instruction.cycles;
        self.data = next_instruction.data;

        return true;
    }

    pub fn x_register(&self) -> isize {
        self.x_register.get()
    }

    pub fn current_cycle(&self) -> usize {
        self.current_cycle
    }
}

#[derive(Clone, Copy)]
pub struct Instruction {
    cycles: usize,
    data: Option<isize>,
    run: fn(&Cpu),
}

impl Instruction {
    pub fn new(cycles: usize, data: Option<isize>, run: fn(&Cpu)) -> Self {
        Self { cycles, data, run }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split_whitespace();
        let instruction = split.next().unwrap();
        let data = split.next();

        Ok(match instruction {
            "addx" => {
                let Some(data) = data else {
                return Err("no data in add instruction".to_string())
            };
                let Ok(data) = data.parse::<isize>() else {
                    return Err("could not parse \"{data}\" to number".to_string())
                };
                Instruction::new(2, Some(data), addx)
            }
            "noop" => Instruction::new(1, None, noop),
            _ => return Err(format!("unknown instruction \"{instruction}\"")),
        })
    }
}

fn addx(cpu: &Cpu) {
    cpu.x_register
        .set(cpu.x_register.get() + cpu.data.expect("no data available for addition"))
}

fn noop(_: &Cpu) {}

fn process_input(input: &str) -> Cpu {
    Cpu::new(
        input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(Instruction::from_str)
            .flat_map(Result::ok),
    )
}

pub fn process_part1(input: &str) -> isize {
    let mut cpu = process_input(input);
    let mut strengths = 0;
    while cpu.clock() {
        // We want the value *before* the cycle
        if cpu.current_cycle >= 19 && (cpu.current_cycle - 19) % 40 == 0 {
            strengths += (cpu.current_cycle() as isize + 1) * cpu.x_register();
        }
    }
    strengths
}

pub fn process_part2(input: &str) {
    let mut cpu = process_input(input);
    for position in 0..240 {
        cpu.clock();
        if cpu.x_register().abs_diff((position + 1) % 40) <= 1 {
            print!("#")
        } else {
            print!(".")
        };
        if (position + 1) % 40 == 0 {
            println!()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{process_part1, process_input};

    #[test]
    fn test_part1_small() {
        let input = "noop\naddx 3\naddx -5";
        let mut cpu = process_input(input);
        let expected_results = [1, 1, 1, 4, 4, -1];
        assert_eq!(expected_results[0], cpu.x_register(), "before 1st clock");
        while !cpu.clock() {
            assert_eq!(expected_results[cpu.current_cycle()], cpu.x_register(), "after clock {}", cpu.current_cycle());
        }
    }

    #[test]
    fn test_part1_1() {
        let input = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";
        assert_eq!(13140, process_part1(input))
    }
}
