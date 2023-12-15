use tracing::info;

use crate::INPUT;

#[derive(Debug)]
struct Lens {
    tag: &'static str,
    focal_length: usize,
}

#[derive(Debug)]
enum Operation {
    Remove,
    Insert { focal_length: usize },
}

#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let v = INPUT
        .split(',')
        .map(|op_str| {
            let op_pos = op_str
                .chars()
                .position(|c: char| !c.is_alphabetic())
                .unwrap();
            let (tag, op_str) = op_str.split_at(op_pos);
            match op_str.chars().next().unwrap() {
                '-' => (tag, Operation::Remove),
                '=' => (
                    tag,
                    Operation::Insert {
                        focal_length: op_str[1..].parse::<usize>().unwrap(),
                    },
                ),
                c => panic!("invalid char: {c}"),
            }
        })
        .collect::<Vec<_>>();
    let mut boxes: [Vec<Lens>; 256] = core::array::from_fn(|_| vec![]);

    for (tag, operation) in v {
        let hash = hash_str(tag);
        let the_box = &mut boxes[hash];
        match operation {
            Operation::Remove => {
                if let Some(idx) = the_box
                    .iter()
                    .position(|v| hash == hash_str(v.tag) && tag == v.tag)
                {
                    the_box.remove(idx);
                }
            }
            Operation::Insert { focal_length } => {
                match the_box
                    .iter()
                    .position(|v| hash == hash_str(v.tag) && tag == v.tag)
                {
                    Some(idx) => the_box[idx] = Lens { tag, focal_length },
                    None => the_box.push(Lens { tag, focal_length }),
                }
            }
        }
    }

    let result = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_idx, the_box)| std::iter::repeat(box_idx).zip(the_box.iter().enumerate()))
        .map(|(box_idx, (slot_idx, lens))| (box_idx + 1) * (slot_idx + 1) * lens.focal_length)
        .sum::<usize>();

    info!(result);

    Ok(())
}

fn hash_str(s: &str) -> usize {
    s.trim()
        .bytes()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
