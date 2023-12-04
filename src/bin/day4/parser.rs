use nom::IResult;


#[derive(Debug, PartialEq)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

impl Card {
   pub  fn get_score(&self) -> u32 {
        let correct = self.winning_numbers
            .iter()
            .filter(|num| self.playing_numbers.contains(num))
            .count() as u32;
        if correct == 0 {
            0
        } else {
            2u32.pow(correct - 1)
        }
    }
}

pub fn parse_card(input: &'static str) -> IResult<&str, Card> {
    let (input, _) = nom::bytes::complete::tag("Card")(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, id) = nom::character::complete::u32(input)?;
    let (input, _) = nom::bytes::complete::tag(":")(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, winning_numbers) = nom::multi::separated_list1(nom::character::complete::space1, nom::character::complete::u32)(input)?;
    let (input, _) = nom::bytes::complete::tag(" |")(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, playing_numbers) = nom::multi::separated_list1(nom::character::complete::space1, nom::character::complete::u32)(input)?;
    Ok((input, Card { id, winning_numbers, playing_numbers }))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            playing_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(parse_card(input).unwrap().1, expected);
    }

    #[test]
    fn could_get_score() {
        let card = Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            playing_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(card.get_score(), 8);
    }
}
