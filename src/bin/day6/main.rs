mod parser;

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

fn solve_part_1(input: &str) -> u64 {
    let rs = parser::parse_game(input).unwrap().1;
    let mut total = 1;
    for r in rs {
        let (least, max) = r.winning_hold_duration();
        total *= max - least + 1
    }
    total
}

fn solve_part_2(input: &str) -> u64 {
    let race = parser::parse_game2(input).unwrap().1;
    let (min, max) = race.winning_hold_duration();
    max - min + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part_1() {
        let input = include_str!("./sample.txt");
        assert_eq!(solve_part_1(input), 288);
    }

    #[test]
    fn could_solve_part_2() {
        let input = include_str!("./sample.txt");
        assert_eq!(solve_part_2(input), 71503);
    }
}
