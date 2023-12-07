use std::collections::HashMap;

use nom::{
    bytes::complete::take_while1,
    character::complete::{self, space1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: &'static str,
    pub bet: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand2(pub Hand);

fn map_to_type_score(map: &HashMap<char, u32>) -> u32 {
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

impl Hand2 {
    fn to_type_number(&self) -> u32 {
        let mut map = HashMap::new();
        for c in self.0.cards.chars() {
            let count = map.entry(c).or_default();
            *count += 1;
        }
        match map.remove(&'J') {
            Some(v) => {
                if v == 5 {
                    return 7;
                }
                let top_key = map
                    .iter()
                    .fold(
                        ('X', 0),
                        |acc, (k, v)| {
                            if *v > acc.1 {
                                (*k, *v)
                            } else {
                                acc
                            }
                        },
                    )
                    .0;
                assert!(top_key != 'X');
                map.entry(top_key).and_modify(|j| *j += v);
            }
            _ => {}
        }
        map_to_type_score(&map)
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.to_type_number().cmp(&other.to_type_number()) {
            std::cmp::Ordering::Equal => {
                for (s, o) in self.0.cards.chars().zip(other.0.cards.chars()) {
                    if char_to_power2(s) == char_to_power2(o) {
                        continue;
                    } else {
                        return char_to_power2(s).cmp(&char_to_power2(o));
                    }
                }
                unreachable!();
            }
            other => other,
        }
    }
}

impl Hand {
    fn to_type_number(&self) -> u32 {
        let mut map = HashMap::new();
        for c in self.cards.chars() {
            let count = map.entry(c).or_default();
            *count += 1;
        }
        map_to_type_score(&map)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.to_type_number().cmp(&other.to_type_number()) {
            std::cmp::Ordering::Equal => {
                for (s, o) in self.cards.chars().zip(other.cards.chars()) {
                    if char_to_power(s) == char_to_power(o) {
                        continue;
                    } else {
                        return char_to_power(s).cmp(&char_to_power(o));
                    }
                }
                unreachable!();
            }
            other => other,
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

fn char_to_power2(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

pub fn parse_hand(input: &'static str) -> IResult<&str, Hand> {
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
