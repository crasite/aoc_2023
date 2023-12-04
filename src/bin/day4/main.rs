mod parser;

fn main(){
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve(input));
}

fn solve(input: &'static str) -> u32 {
    let mut cards = Vec::new();
    for line in input.lines() {
        let card = parser::parse_card(line).unwrap().1;
        cards.push(card);
    }
    cards.iter().map(|card| card.get_score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part1() {
        let input = include_str!("sample.txt");
        assert_eq!(solve(input), 13);
    }
}
