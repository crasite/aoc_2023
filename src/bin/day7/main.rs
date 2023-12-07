mod parser;

fn main() {
    let input = include_str!("./input.txt");
    println!("Part1: {}", solve_part1(input));
}

fn solve_part1(input: &'static str) -> i64 {
    let mut total_winnings = 0;
    let mut hands = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let hand = parser::parse_hand(line).unwrap().1;
        hands.push(hand);
    }
    hands.sort();
    for i in 1..=hands.len() {
        total_winnings += hands[i - 1].bet * i as i64;
    }
    total_winnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part1() {
        let input = include_str!("./sample.txt");
        let expect = 6440;
        assert_eq!(solve_part1(input), expect);
    }
}
