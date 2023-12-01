mod parser;

fn main()  {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve(input));
    println!("Part 2: {}", solve2(input));
}

fn solve(input: &str) -> u32 {
    input.lines().map(|line| parser::parse(line).unwrap().1).sum()
}

fn solve2(input: &str) -> u32 {
    input.lines().map(|line| parser::parse2(line).unwrap().1).sum()
}
