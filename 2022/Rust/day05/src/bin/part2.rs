fn main() {
    let input = include_str!("../../input.txt");

    let mut temp_stack = vec![];

    let (mut boxes, instructions) = day05::parse_input(input);
    for instruction in instructions {
        for _ in 0..instruction.num_boxes {
            if let Some(temp) = boxes[instruction.source].pop() {
                temp_stack.push(temp)
            }
        }
        while let Some(temp) = temp_stack.pop() {
            boxes[instruction.target].push(temp);
        }
    }

    println!(
        "{}",
        boxes.iter().filter_map(|v| v.last()).collect::<String>()
    )
}
