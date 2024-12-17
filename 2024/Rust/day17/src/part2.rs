use tracing::info;

use crate::{eval_input, make_input, parse_input, Opcode, INPUT, SAMPLE, SAMPLE2, SAMPLE3};

// TOO HIGH???????? IT SOLVES THE FUCKING TASK 108753730021786

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let program = parse_input(&make_input(0))
        .program
        .into_iter()
        .map(|v| v as u8)
        .collect::<Vec<_>>();

    let mut result = find_start_value(0, &program);
    /*
    'out: for v in program.iter().rev().copied() {
        result <<= 3;
        info!(v);
        for i in 0..8 {
            let mut cpu = parse_input(&make_input(result | i));

            while let Some(op) = cpu.tick() {
                if op == Opcode::Out {
                    break;
                }
            }
            info!(i, o = cpu.output_buffer[0]);
            if cpu.output_buffer[0] == v {
                info!(i);
                eval_input(&make_input(result));
                continue 'out;
            }
        }
        panic!("{v}")
    }
*/
    info!(result);
    Ok(())
}

fn find_start_value(prev: usize, program: &[u8]) -> Option<usize> {
    if program.is_empty() {
        return Some(prev);
    }

    let prev = prev << 3;
    for i in 0..8 {
        let mut cpu = parse_input(&make_input(prev | i));

        while let Some(op) = cpu.tick() {
            if op == Opcode::Out {
                break;
            }
        }
        info!(i, o = cpu.output_buffer[0]);
        if cpu.output_buffer[0] == *program.last().unwrap() {
            if let Some(s) = find_start_value(prev | i, &program[..program.len() - 1]) {
                eval_input(&make_input(s));
                return Some(s);
            }
        }
    }
    None
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
