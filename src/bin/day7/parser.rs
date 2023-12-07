use std::collections::HashMap;

use nom::{
    bytes::complete::take_while1,
    character::complete::{self, space1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
struct Hand {
    cards: &'static str,
    bet: i64,
}

impl Hand {
    fn to_type_number(&self) -> u32 {
        let mut map = HashMap::new();
        for c in self.cards.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        if map.iter().any(|(_, v)| *v == 5) {
            7
        } else if map.iter().any(|(_, v)| *v == 4) {
            6
        } else if map.iter().any(|(_, v)| *v == 3) && map.iter().any(|(_, v)| *v == 2) {
            5
        } else if map.iter().any(|(_, v)| *v == 3) {
            4
        } else if map.iter().filter(|(_, v)| **v == 2).count() == 2 {
            3
        } else if map.iter().any(|(_, v)| *v == 2) {
            2
        } else {
            1
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.to_type_number().cmp(&other.to_type_number()) {
            std::cmp::Ordering::Equal => {
                let other = other.cards.chars().collect::<Vec<_>>();
                for (i, c) in self.cards.chars().enumerate() {
                    if char_to_power(c) == char_to_power(other[i]) {
                        continue;
                    } else {
                        return Some(char_to_power(c).cmp(&char_to_power(other[i])));
                    }
                }
                unreachable!();
            }
            other => Some(other),
        }
    }
}

fn char_to_power(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

fn parse_hand(input: &'static str) -> IResult<&str, Hand> {
    let (input, (cards, hand)) = separated_pair(
        take_while1(|c: char| !c.is_whitespace()),
        space1,
        complete::i64,
    )(input)?;
    Ok((input, Hand { cards, bet: hand }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_hand() {
        let input = "32T3K 765";
        let expect = Hand {
            cards: "32T3K",
            bet: 765,
        };

        assert_eq!(parse_hand(input), Ok(("", expect)));
    }

    #[test]
    fn could_compare_hand() {
        let hand1 = parse_hand("QAAQT 665").unwrap().1;
        let hand2 = parse_hand("5K355 312").unwrap().1;
        assert!(hand1 < hand2);
    }
}
