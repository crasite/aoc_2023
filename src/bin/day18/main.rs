use std::collections::{HashMap};

mod model;
mod parser;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve(input));
    println!("Part 2: {}", solve2(input));
}

fn solve(input: &'static str) -> i32 {
    let mut droplets = Vec::new();
    for line in input.lines() {
        let droplet = parser::parse_droplet(line).unwrap().1;
        droplets.push(droplet);
    }
    let mut empty_sides = 0;
    for droplet in &droplets {
        empty_sides += droplet.get_empty_side(&droplets)
    }
    empty_sides
}

fn solve2(input: &'static str) -> i32 {
    let mut droplets = Vec::new();
    for line in input.lines() {
        let droplet = parser::parse_droplet(line).unwrap().1;
        droplets.push(droplet);
    }
    let mut empty_sides = 0;
    let (max_x, max_y, max_z) = model::max_dimension(&droplets);
    let mut cache = HashMap::new();
    for droplet in &droplets {
        let air_droplets = droplet.get_empty_droplets(&droplets);
        empty_sides += air_droplets
            .iter()
            .filter(|drop| !drop.is_enclosed(&droplets, max_x, max_y, max_z, Some(&mut cache)))
            .count();
    }
    empty_sides as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part1() {
        let input = include_str!("sample.txt");
        assert_eq!(solve(input), 64);
    }

    #[test]
    fn could_solve_part2() {
        let input = include_str!("sample.txt");
        assert_eq!(solve2(input), 58);
    }
}
