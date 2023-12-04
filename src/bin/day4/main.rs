use std::collections::{HashMap};

mod parser;

fn main(){
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve(input));
    println!("Part 2: {}", solve2(input));
}

fn solve(input: &'static str) -> u32 {
    let mut cards = Vec::new();
    for line in input.lines() {
        let card = parser::parse_card(line).unwrap().1;
        cards.push(card);
    }
    cards.iter().map(|card| card.get_score()).sum()
}

fn solve2(input: &'static str) -> u32 {
    let mut cards = Vec::new();
    let mut card_map = HashMap::new();
    for line in input.lines() {
        let card = parser::parse_card(line).unwrap().1;
        cards.push(card);
    }
    let max = cards.len();
    for card in &cards {
        match card_map.get(&card.id) {
            Some(v) => card_map.insert(card.id, v + 1),
            None => card_map.insert(card.id, 1),
        };
        let next_cards = card.get_next_cards();
        let total = *card_map.get(&card.id).unwrap();
        for next_card in next_cards {
            if next_card > max as u32 {
                break;
            }
            match card_map.get(&next_card) {
                Some(v) => card_map.insert(next_card, v + total),
                None => card_map.insert(next_card, total),
            };
        };
    }
    card_map.iter().fold(0, |acc, (_, v)| acc + v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part1() {
        let input = include_str!("sample.txt");
        assert_eq!(solve(input), 13);
    }

    #[test]
    fn could_solve_part2() {
        let input = include_str!("sample.txt");
        assert_eq!(solve2(input), 30);
    }
}
