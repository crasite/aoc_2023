use std::collections::HashMap;

mod parser;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &'static str) -> i64 {
    let (mut instruction, nodes) = parser::parse(input).unwrap().1;
    let mut map = HashMap::new();
    for node in nodes {
        map.insert(node.name, (node.left, node.right));
    }
    let mut current_node = "AAA";
    let mut total_move = 0;
    while current_node != "ZZZ" {
        let (left, right) = map.get(current_node).unwrap();
        match instruction.next() {
            'L' => current_node = left,
            'R' => current_node = right,
            _ => unreachable!(),
        }
        total_move += 1;
    }
    return total_move;
}

fn part2(input: &'static str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part1() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 2);
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 6);
    }
}
