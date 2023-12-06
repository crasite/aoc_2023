mod parser;

fn main(){
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
}

fn solve_part_1(input: &str) -> i32 {
    let rs = parser::parse_game(input).unwrap().1;
    let mut total = 1;
    for r in rs {
        let (least, max) = r.winning_hold_duration();
        total *= max-least+1
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part_1() {
        let input = include_str!("./sample.txt");
        assert_eq!(solve_part_1(input), 288);
    }
}
