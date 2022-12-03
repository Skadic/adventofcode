#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum FoldAlong {
    X(usize),
    Y(usize),
}

fn main() {
    let input = include_str!("../input.txt");
    
    println!("Part 1: {}", part1(input));
    println!("Part 2:");
    part2(input)
}

fn part1(input: &str) -> usize {
    let (mut map, folds) = process_input(input);
    
    fold(&mut map, folds[0]);
    map.into_iter().flat_map(|v| v).filter(|&b| b).count()
}

fn part2(input: &str) {
    let (mut map, folds) = process_input(input);
    
    for f in folds {
        fold(&mut map, f);
    }
    print_map(&map);
}

fn print_map(map: &[Vec<bool>]) {
    for row in map {
        for &dot in row {
            if dot {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn fold(map: &mut Vec<Vec<bool>>, fold: FoldAlong){
    let width = map[0].len();
    match fold {
        FoldAlong::Y(fold_idx) => {
            for y in 1..fold_idx + 1 {
                let (target_space, source_space) = map.split_at_mut(fold_idx + 1);
                for x in 0..width {
                    let Some(target) =  target_space.get_mut(fold_idx - y).and_then(|row| row.get_mut(x)) else {
                        continue;
                    };
                    let Some(source) =  source_space.get(y - 1).and_then(|row| row.get(x)) else {
                        continue;
                    };
                    *target |= *source;
                }
            }
            map.truncate(fold_idx + 1);
        }
        FoldAlong::X(fold_idx) => {
            for y in 0..map.len() {
                for x in 1..fold_idx + 1 {
                    let (target_space, source_space) = map[y].split_at_mut(fold_idx + 1);
                    let Some(target) = target_space.get_mut(fold_idx - x) else {
                        continue;
                    };
                    let Some(source) =source_space.get(x - 1) else {
                        continue;
                    };
                    *target |= *source;
                }
                map[y].truncate(fold_idx + 1);
            }
        }
    }
}

fn process_input(input: &str) -> (Vec<Vec<bool>>, Vec<FoldAlong>) {
    let map = {
        let points: Vec<(usize, usize)> = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .take_while(|l| l.chars().next().unwrap().is_ascii_digit())
            .map(|l| {
                let mut split = l.split(',');
                (
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        let (max_x, max_y) = points.iter().fold((0, 0), |(old_x, old_y), (x, y)| {
            (old_x.max(*x), old_y.max(*y))
        });

        let mut v = (0..max_y + 1)
            .map(|_| vec![false; max_x + 1])
            .collect::<Vec<_>>();
        for (x, y) in points {
            v[y][x] = true;
        }
        v
    };

    let folds = input
        .lines()
        .skip_while(|line| !line.starts_with("fold"))
        .map(|line| {
            let mut tokens = line.chars().skip_while(|&c| c != 'x' && c != 'y');
            let direction = tokens.next().unwrap();
            tokens.next().unwrap();
            let id = tokens.collect::<String>().parse::<usize>().unwrap();
            match direction {
                'x' => FoldAlong::X(id),
                'y' => FoldAlong::Y(id),
                _ => panic!("Invalid fold direction: {direction}"),
            }
        })
        .collect::<Vec<_>>();

    (map, folds)
}
