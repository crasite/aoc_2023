use nom::IResult;

#[derive(Debug, PartialEq)]
struct Hand {
    card: &'static str,
    bet: i64,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

fn parse_hand(input: &'static str) -> IResult<&str, Hand> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_hand() {
        let input = "32T3K 765";
        let expect = Hand {
            card: "32T3K",
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
