use std::{cmp::min_by_key, collections::VecDeque, iter::once};

use itertools::Itertools as _;
use tracing::info;

use crate::{
    actions_possible_moves, move_to, num_from_pos, parse_input, travel_strs, travel_strs_numpad,
    RobotAction, INPUT, NUMPAD, NUMPAD_GAP_X, NUMPAD_GAP_Y, SAMPLE,
};

//#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let map = actions_possible_moves();

    let actions = parse_input(INPUT);

    let mut result = 0;
    for (name, v) in actions {
        let s = neext_numpad(v);

        info!(?s);
        let s = s.into_iter().flat_map(|v| neext(&v)).collect_vec();
        let s = s
            .into_iter()
            .flat_map(|v| neext(&v))
            .min_by_key(|v| v.len())
            .unwrap();

        /*
        let s = once('A')
            .chain(s.chars())
            .tuple_windows::<(_, _)>()
            .flat_map(|(prev, next)| travel_strs(prev, next).as_str().chars().chain(once('A')))
            .collect::<String>();
        info!(s);

        let s = once('A')
            .chain(s.chars())
            .tuple_windows::<(_, _)>()
            .map(|(prev, next)| (prev.to_string(), next.to_string()))
            .flat_map(|(prev, next)| map[&prev][&next][0].as_str().chars().chain(once('A')))
            .collect::<String>();
        info!(?s);
        */
        let numeric_part = usize::from_str_radix(name, 10).unwrap();
        result += numeric_part * s.len();
    }
    info!(result);

    Ok(())
}

fn neext(s: &str) -> Vec<String> {
    #[derive(Debug)]
    struct State<'a> {
        st: String,
        rest: &'a str,
    }
    let s = once('A').chain(s.chars()).collect::<String>();

    let mut v = VecDeque::new();
    v.push_back(State {
        st: "".to_string(),
        rest: &s,
    });

    let mut finished_strings = Vec::new();

    while let Some(state) = v.pop_back() {
        let mut it = state.rest.chars();
        let Some(from) = it.next() else {
            continue;
        };
        let Some(to) = it.next() else {
            finished_strings.push(state.st);
            continue;
        };
        let strs = travel_strs(from, to);

        v.extend(strs.into_iter().map(|next| State {
            st: format!("{}{}A", &state.st, next),
            rest: &state.rest[1..],
        }));
    }

    let min = finished_strings.iter().map(|s| s.len()).min().unwrap();
    finished_strings
        .into_iter()
        .filter(|s| s.len() == min)
        .collect_vec()
}

fn neext_numpad(s: &str) -> Vec<String> {
    #[derive(Debug)]
    struct State<'a> {
        st: String,
        rest: &'a str,
    }
    let s = once('A').chain(s.chars()).collect::<String>();

    let mut v = VecDeque::new();
    v.push_back(State {
        st: "".to_string(),
        rest: &s,
    });

    let mut finished_strings = Vec::new();

    while let Some(state) = v.pop_back() {
        let mut it = state.rest.chars();
        let Some(from) = it.next() else {
            continue;
        };
        let Some(to) = it.next() else {
            finished_strings.push(state.st);
            continue;
        };
        let strs = travel_strs_numpad(from, to);

        v.extend(strs.into_iter().map(|next| State {
            st: format!("{}{}A", &state.st, next),
            rest: &state.rest[1..],
        }));
    }

    let min = finished_strings.iter().map(|s| s.len()).min().unwrap();
    finished_strings
        .into_iter()
        .filter(|s| s.len() == min)
        .collect_vec()
}

fn find_instructions<T>(
    input_sequence: impl IntoIterator<Item = T>,
    sx: usize,
    sy: usize,
    gap_x: usize,
    gap_y: usize,
    find_pos: impl Fn(T) -> (usize, usize),
    depth: usize,
) -> String {
    let instr = once((sx, sy))
        .chain(input_sequence.into_iter().map(find_pos))
        .tuple_windows::<(_, _)>()
        .map(|((sx, sy), (tx, ty))| {
            let (mut actions, both_swaps_possible) = move_to(sx, sy, tx, ty, gap_x, gap_y);
            let swap1 = actions
                .clone()
                .into_iter()
                .flat_map(|(action, num)| std::iter::repeat(action.into()).take(num))
                .chain(once('A'))
                .collect::<String>();
            let swap2 = if both_swaps_possible {
                actions.swap(0, 1);
                actions
                    .clone()
                    .into_iter()
                    .flat_map(|(action, num)| std::iter::repeat(action.into()).take(num))
                    .chain(once('A'))
                    .collect::<String>()
            } else {
                return swap1;
            };

            min_by_key(swap1, swap2, |v: &String| v.len())
        })
        .collect();
    instr
}

fn reconstruct_actions(actions: impl IntoIterator<Item = char>) -> String {
    let mut state = RobotAction::Activate.get_pos();
    let mut output = String::new();
    for c in actions.into_iter() {
        info!(%c, ?state);
        match RobotAction::from(c) {
            RobotAction::Up => state.1 -= 1,
            RobotAction::Down => state.1 += 1,
            RobotAction::Left => state.0 -= 1,
            RobotAction::Right => state.0 += 1,
            RobotAction::Activate => output.push(RobotAction::from(state).into()),
        };
    }
    output
}

fn reconstruct_code(actions: impl IntoIterator<Item = char>) -> String {
    let mut state = NUMPAD[10];
    let mut output = String::new();
    for c in actions.into_iter() {
        match RobotAction::from(c) {
            RobotAction::Up => state.1 -= 1,
            RobotAction::Down => state.1 += 1,
            RobotAction::Left => state.0 -= 1,
            RobotAction::Right => state.0 += 1,
            RobotAction::Activate => output.push(num_from_pos(state)),
        };
    }
    output
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
