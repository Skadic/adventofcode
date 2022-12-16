use day14::{print_map, process_input, MapElem};

fn main() {
    let input = include_str!("../../input.txt");
    process_part1(input);
}

fn process_part1(input: &str) -> usize {
    let mut map = process_input(input);
    let sand_source = (500, 0);

    let mut count = 0;
    loop {
        let mut sand = sand_source;
        let mut settled = false;
        loop {
            println!("{:?}", sand);

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

            settled = true;
            break;
        }

        if !settled {
            break;
        }

        map.set(sand.0, sand.1, MapElem::Sand);
        count += 1;
    }

    println!("----------------------------------");
    print_map(&map);

    println!("Part 1: {count}");
    count
}

#[cfg(test)]
mod test {
    use crate::process_part1;

    #[test]
    fn test() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(24, process_part1(input));
    }
}
