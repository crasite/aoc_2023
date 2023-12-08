use std::collections::HashMap;

use rayon::prelude::*;

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

fn is_done(current_nodes: &[&str]) -> bool {
    for node in current_nodes {
        if node.chars().last().unwrap() != 'Z' {
            return false;
        }
    }
    true
}

fn part2(input: &'static str) -> i64 {
    let (mut instruction, nodes) = parser::parse(input).unwrap().1;
    let mut map = HashMap::new();
    for node in nodes {
        map.insert(node.name, (node.left, node.right));
    }
    let mut current_nodes = vec![];
    for node in map.keys() {
        if node.chars().last().unwrap() == 'A' {
            current_nodes.push(*node);
        }
    }
    let mut total_move = 0;

    while !is_done(&current_nodes) {
        let direction = instruction.next();
        current_nodes.par_iter_mut().for_each(|node| {
            let (left, right) = map.get(*node).unwrap();
            match direction {
                'L' => *node = *left,
                'R' => *node = *right,
                _ => unreachable!(),
            }
        });
        total_move += 1;
        if (total_move % 10000) == 0 {
            println!("{}: {:?}", total_move, current_nodes);
        }
    }
    total_move
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

    #[test]
    fn could_solve_part2() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2(input), 6);
    }
}
