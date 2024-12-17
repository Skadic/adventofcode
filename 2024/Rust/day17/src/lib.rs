use tracing::info;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

pub const SAMPLE2: &str = "Register A: 117440 
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

pub const SAMPLE3: &str = "Register A: 10
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,1,6,4,1,5,5,0,3,3,0
"; //     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5

pub fn make_input(start: usize) -> String {
    format!(
        "Register A: {start}\nRegister B: 0\nRegister C: 0\n\nProgram: 2,4,1,5,7,5,1,6,4,1,5,5,0,3,3,0"
    
    )
}

/*

Bst A
Bxl 5
Cdv B
Bxl 6
Bxc 1
Out B -> 5
Adv 3
Jnz 0


B = A % 8
B' = B ^ 101
C = A >> B'
B'' = B' ^ 110
B''' = B'' ^ C
Print B'''
A' = A >> 3
if A' = 0 stop



B''' = (A % 8) ^ 011 ^ (((A' << 3) | (A % 8)) >> ((A % 8) ^ 101))


*/

/*
B''' = B'' ^ (A >> B')
B''' = (A & 111) ^ 011 ^ (A >> ((A & 111) ^ 101))
<=> B'' = B''' ^ C
<=> B'' =

*/

pub mod part1;
pub mod part2;

const MOD8_MASK: usize = 0b111;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction(Opcode, AddressingMode);

pub const INSTRUCTIONS: [Instruction; 8] = [
    Instruction(Opcode::Adv, AddressingMode::Combo),
    Instruction(Opcode::Bxl, AddressingMode::Literal),
    Instruction(Opcode::Bst, AddressingMode::Combo),
    Instruction(Opcode::Jnz, AddressingMode::Literal),
    Instruction(Opcode::Bxc, AddressingMode::Literal),
    Instruction(Opcode::Out, AddressingMode::Combo),
    Instruction(Opcode::Bdv, AddressingMode::Combo),
    Instruction(Opcode::Cdv, AddressingMode::Combo),
];

pub struct Cpu {
    a: usize,
    b: usize,
    c: usize,
    last_c_shift: usize,
    program: Vec<usize>,
    pc: usize,
    clock: usize,
    pub output_buffer: Vec<u8>,
}

impl Cpu {
    pub fn tick(&mut self) -> Option<Opcode> {
        let Instruction(opcode, adressing_mode): Instruction = self
            .program
            .get(self.pc)
            .copied()
            .map(|i| INSTRUCTIONS[i])?;
        self.pc += 1;
        let operand: usize = adressing_mode.read(self)?;
        if opcode == Opcode::Cdv {
            self.last_c_shift = operand;
        }
        /*
        if opcode == Opcode::Out {
            info!(
                "{:#4}: {opcode:?} {} -> {}",
                self.clock,
                adressing_mode.fmt(self.program[self.pc]),
                operand % 8
            );
        } else {
            info!(
                "{:#4}: {opcode:?} {}",
                self.clock,
                adressing_mode.fmt(self.program[self.pc])
            );
        }
*/
        self.pc += 1;

        self.execute(opcode, operand);
        self.clock += 1;
        Some(opcode)
    }

    fn execute(&mut self, opcode: Opcode, operand: usize) {
        match opcode {
            Opcode::Adv => {
                self.a >>= operand;
            }
            Opcode::Bxl => {
                self.b ^= operand;
            }
            Opcode::Bst => self.b = operand & MOD8_MASK,
            Opcode::Jnz => {
                if self.a != 0 {
                    self.pc = operand;
                }
            }
            Opcode::Bxc => {
                self.b ^= self.c;
            }
            Opcode::Out => self.output_buffer.push((operand & MOD8_MASK) as u8),
            Opcode::Bdv => {
                self.b = self.a >> operand;
            }
            Opcode::Cdv => {
                self.c = self.a >> operand;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode: {value}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Literal,
    Combo,
}

impl AddressingMode {
    fn read(self, cpu: &Cpu) -> Option<usize> {
        Some(match self {
            Self::Literal => cpu.program.get(cpu.pc).copied()?,
            Self::Combo => match cpu.program[cpu.pc] {
                0..=3 => cpu.program[cpu.pc],
                4 => cpu.a,
                5 => cpu.b,
                6 => cpu.c,
                _ => panic!("invalid combo addressing: {}", cpu.program[cpu.pc]),
            },
        })
    }

    fn fmt(self, operator: usize) -> char {
        match self {
            Self::Literal => char::from_digit(operator as u32, 10).unwrap(),
            Self::Combo => match operator {
                0..=3 => char::from_digit(operator as u32, 10).unwrap(),
                4 => 'A',
                5 => 'B',
                6 => 'C',
                _ => panic!("invalid combo addressing: {operator}"),
            },
        }
    }
}

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Cpu {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut regs = registers
        .lines()
        .map(|line| line[12..].trim().parse::<usize>().unwrap());
    let a = regs.next().unwrap();
    let b = regs.next().unwrap();
    let c = regs.next().unwrap();

    let program = program[9..]
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    Cpu {
        a,
        b,
        c,
        last_c_shift: 0,
        program,
        pc: 0,
        clock: 0,
        output_buffer: vec![],
    }
}

pub fn eval_input(input: &str) {
    let mut cpu = parse_input(input);

    while let Some(_) = cpu.tick() {
    }

    let s = cpu
        .output_buffer
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    info!(result = s);
}
