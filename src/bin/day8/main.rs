use std::collections::HashMap;

use num::integer::lcm;

use parser::Instruction;
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

#[derive(Debug, Clone, Copy)]
struct Record {
    current: i64,
    loop_size: i64,
}

fn get_node_loop(
    current_nodes: &str,
    instruction: &mut Instruction,
    map: &HashMap<&str, (&str, &str)>,
) -> Vec<Record> {
    let base_node = current_nodes;
    let mut passed_node = vec![];

    passed_node.push((base_node, instruction.index));
    let mut current_node = move_node(map, base_node, instruction);
    while !passed_node.contains(&(current_node, instruction.index)) {
        passed_node.push((current_node, instruction.index));
        let (left, right) = map.get(current_node).unwrap();
        match instruction.next() {
            'L' => current_node = left,
            'R' => current_node = right,
            _ => unreachable!(),
        }
    }
    let index = passed_node
        .iter()
        .position(|&x| x == (current_node, instruction.index))
        .unwrap();
    let loop_size = passed_node.len() - index;
    let mut records = vec![];
    for i in 0..passed_node.len() {
        if passed_node[i].0.chars().last().unwrap() == 'Z' {
            records.push(Record {
                current: i as i64,
                loop_size: loop_size as i64,
            });
        }
    }
    records
}

fn move_node<'a>(
    map: &HashMap<&str, (&'a str, &'a str)>,
    current_node: &str,
    instruction: &mut Instruction,
) -> &'a str {
    let (left, right) = map.get(current_node).unwrap();
    match instruction.next() {
        'L' => *left,
        'R' => *right,
        _ => unreachable!(),
    }
}

fn part2(input: &'static str) -> i64 {
    let (instruction, nodes) = parser::parse(input).unwrap().1;
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
    let mut records = vec![];
    for node in current_nodes.iter() {
        let mut instruction = instruction.clone();
        records.push(
            get_node_loop(node, &mut instruction, &map)
                .last()
                .unwrap()
                .clone(),
        );
    }
    records.iter().fold(1, |acc, x| lcm(acc, x.loop_size))
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
