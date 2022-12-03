use std::collections::HashSet;

pub fn item_priority(item: char) -> usize {
    if item.is_uppercase() {
        item as usize - 'A' as usize + 1 + 26
    } else {
        item as usize - 'a' as usize + 1
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|items| {
            let items = items.trim();
            let (first_compartment_items, second_compartment_items) =
                items.split_at(items.len() / 2);
            let first_compartment_items_distinct =
                first_compartment_items.chars().collect::<HashSet<_>>();
            let duplicate_item = second_compartment_items
                .chars()
                .find(|item| first_compartment_items_distinct.contains(item))
                .expect("elf actually did his work lol");
            item_priority(duplicate_item)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|rucksacks| {
            let mut elves = [
                rucksacks[0].chars().collect::<Vec<_>>(),
                rucksacks[1].chars().collect::<Vec<_>>(),
                rucksacks[2].chars().collect::<Vec<_>>(),
            ];

            for elf in elves.iter_mut() {
                elf.sort_unstable()
            }

            let mut indices = [0, 0, 0];

            macro_rules! elf_item {
                ($i:literal) => {
                    elves[$i][indices[$i]]
                };
            }

            let duplicate_item = loop {
                if elf_item!(0) == elf_item!(1) && elf_item!(1) == elf_item!(2) {
                    break elf_item!(0);
                }

                let (elf_with_lowest_item, _) = [elf_item!(0), elf_item!(1), elf_item!(2)]
                    .into_iter()
                    .enumerate()
                    .min_by_key(|(_, item)| *item)
                    .unwrap();

                indices[elf_with_lowest_item] += 1;
            };

            item_priority(duplicate_item)
        })
        .sum()
}
