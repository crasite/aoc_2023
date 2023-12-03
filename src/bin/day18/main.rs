mod parser;
mod model;

fn main(){
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve(input));
    // println!("Part 2: {}", solve2(input));
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
    todo!()
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
        assert_eq!(solve(input), 64);
    }
}
