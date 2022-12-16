
use day13::{process_input, PacketElem};

fn main() {
    use PacketElem::*;
    let input = include_str!("../../input.txt");
    let mut packets = process_input(input).into_iter().flat_map(|(l, r)| [l, r]).collect::<Vec<_>>();
    let dividers = vec![List(vec![List(vec![Packet(6)])]), List(vec![List(vec![Packet(2)])])];

    packets.extend(dividers.clone());
    packets.sort_unstable();
    let mut res = 1;
    for (i, l) in packets.into_iter().enumerate() {
        if dividers.contains(&l) {
            res *= i + 1;
        }
    }
    
    println!("Part 2: {res}")
}
