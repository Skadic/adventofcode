use std::collections::HashMap;

fn main() {
    let input = include_str!("../res/input.txt");

    let mut signal_patterns = input
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            split.next().unwrap()
        })
        .map(|sig| sig.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for pattern in signal_patterns.iter_mut() {
        pattern.sort_unstable_by(|&a, &b| a.len().cmp(&b.len()));
    }

    let output_patterns = input
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            split.nth(1).unwrap()
        })
        .map(|sig| sig.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result = part12(&signal_patterns, &output_patterns);

    println!(
        "Part 1: {}",
        result
            .iter()
            .flat_map(|arr| arr.iter())
            .cloned()
            .filter(|digit| [1, 4, 7, 8].contains(digit))
            .count()
    );
    println!(
        "Part 1: {}",
        result
            .iter()
            .map(|arr| arr
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &digit)| digit * 10usize.pow(i as u32))
                .sum::<usize>())
            .sum::<usize>()
    );
}

fn part12(signal_patterns: &[Vec<&str>], output_patterns: &[Vec<&str>]) -> Vec<[usize; 4]> {
    let mut result_vec = vec![];

    for (pattern_vec, output_vec) in signal_patterns.iter().zip(output_patterns.iter()) {
        let mut mapping = HashMap::new();

        // calculate 'a'
        {
            let a = pattern_vec[1]
                .chars()
                .filter(|&c| !pattern_vec[0].contains(c))
                .next()
                .unwrap();

            mapping.insert('a', a);
        }

        // calculate 'c' and 'f'
        {
            let cf_possible = pattern_vec[0];

            let f = pattern_vec[6..=8] // Elements with 6 segments (0, 6, 9)
                .iter()
                .map(|pattern| {
                    pattern
                        .chars()
                        .filter(|&c| cf_possible.contains(c))
                        .collect::<Vec<_>>()
                })
                .filter(|chars| chars.len() == 1) // This should give us the one segment that 6 shares with 1 ('f')
                .next()
                .unwrap()[0];

            let c = pattern_vec[0].chars().filter(|&c| c != f).next().unwrap();

            mapping.insert('f', f);
            mapping.insert('c', c);
        }

        // calculate 'g'

        {
            let pattern9 = pattern_vec[6..=8]
                .iter()
                .cloned()
                .filter(|pat| pattern_vec[2].chars().all(|c| pat.contains(c))) // The only pattern with 6 segments that contains all segments from 4, is 9
                .next()
                .unwrap();

            let g = pattern9
                .chars()
                .filter(|&c| c != mapping[&'a']) // remove segment 'a'
                .filter(|&c| !pattern_vec[2].contains(c)) // remove all segments in '4'
                .next() // We should be left with the only remaining segment in '9', that being 'g'
                .unwrap();

            mapping.insert('g', g);
        }

        // calculate 'd'

        {
            let d = pattern_vec[3..=5] // The patterns with 5 segments
                .iter()
                .cloned()
                .filter(|&pattern| mapping.values().all(|&c| pattern.contains(c))) // We have segments 'a', 'c', 'f' and 'g' so far. Only 3 contains those out of the digits with 5 segments
                .flat_map(|pattern3| pattern3.chars())
                .filter(|&c| !mapping.values().any(|&c2| c == c2))
                .next()
                .unwrap();

            mapping.insert('d', d);
        }

        // calculate 'b'

        {
            let b = pattern_vec[2] // get 4
                .chars()
                .filter(|&c| !mapping.values().any(|&c2| c == c2))
                .next()
                .unwrap();

            mapping.insert('b', b);
        }

        // calculate 'e'

        {
            let e = pattern_vec[9] // get 8
                .chars()
                .filter(|&c| !mapping.values().any(|&c2| c == c2))
                .next()
                .unwrap();

            mapping.insert('e', e);
        }

        // map back to the original segments
        let reverse_mapping = mapping
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<HashMap<_, _>>();

        // map each output back to the original segments and
        let mut outputs_sorted = output_vec
            .iter()
            .map(|&s| s.chars().map(|c| reverse_mapping[&c]).collect::<String>())
            .collect::<Vec<_>>();

        // sort each output string alphabetically
        for s in outputs_sorted.iter_mut() {
            // SAFETY: The only characters in the strings are ASCII, so they will always be valid utf8 if the bytes are sorted
            unsafe { s.as_mut_vec() }.sort_unstable();
        }

        let mut res_iter = outputs_sorted.into_iter().map(|s| match &s[..] {
            "abcefg" => 0usize,
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            other => panic!("Invalid digit pattern: {}", other),
        });

        result_vec.push([
            res_iter.next().unwrap(),
            res_iter.next().unwrap(),
            res_iter.next().unwrap(),
            res_iter.next().unwrap(),
        ])
    }

    result_vec
}
