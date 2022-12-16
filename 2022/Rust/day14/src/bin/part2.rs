use day14::{process_input, MapElem};

fn main() {
    let input = include_str!("../../input.txt");
    process_part2(input);
}

fn process_part2(input: &str) -> usize {
    let mut map = process_input(input);
    let sand_source = (500, 0);

    for i in 0..map.width() {
        map.set(i, map.height() - 1, MapElem::Rock);
    }

    let mut count = 0;
    loop {
        let mut sand = sand_source;
        loop {
            if sand.1 == map.height() - 1 {
                println!("a");
                break;
            }

            if !map.get(sand.0, sand.1 + 1).is_solid() {
                sand = (sand.0, sand.1 + 1);
                continue;
            }

            if sand.0 == 0 {
                println!("b");
                break;
            }

            if !map.get(sand.0 - 1, sand.1 + 1).is_solid()
                && (!map.get(sand.0 - 1, sand.1).is_solid()
                    || map.get(sand.0, sand.1 + 1).is_sand())
            {
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            }

            if sand.0 == map.width() - 1 {
                println!("c");
                break;
            }

            if !map.get(sand.0 + 1, sand.1 + 1).is_solid()
                && (!map.get(sand.0 + 1, sand.1).is_solid()
                    || map.get(sand.0, sand.1 + 1).is_sand())
            {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }

            break;
        }

        if sand == sand_source {
            count += 1;
            break;
        }

        map.set(sand.0, sand.1, MapElem::Sand);
        count += 1;
    }

    println!("Part 2: {count}");
    count
}

#[cfg(test)]
mod test {
    use crate::process_part2;

    #[test]
    fn test() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(93, process_part2(input));
    }
}
