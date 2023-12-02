use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{multispace0, u32 as parse_u32},
    combinator::{eof, not, opt},
    multi::many1,
    IResult,
};

pub fn parse_game(input: &'static str) -> Result<Game> {
    let (_, game) = parse_game_line(input).map_err(|e| anyhow!(e))?;
    Ok(game)
}

fn parse_game_line(input: &'static str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = parse_u32(input)?;
    let (input, hints) = many1(parse_hints)(input)?;
    Ok((
        input,
        Game {
            id: id as usize,
            hint: hints,
        },
    ))
}

fn parse_hint(input: &'static str) -> IResult<&str, Hint> {
    let (input, _) = take_till(|c: char| c.is_numeric())(input)?;
    let (input, count) = parse_u32(input)?;
    let (input, _) = multispace0(input)?;
    let (input, color) = alt((tag("blue"), tag("green"), tag("red")))(input)?;
    Ok((
        input,
        Hint {
            color,
            count: count as usize,
        },
    ))
}

fn parse_hints(input: &'static str) -> IResult<&str, Vec<Hint>> {
    let (input, _) = not(eof)(input)?;
    let mut input = input;
    let mut hints = Vec::new();
    while !input.is_empty() {
        let (loop_input, hint) = parse_hint(input)?;
        hints.push(hint);
        let (loop_input, end) = opt(tag(";"))(loop_input)?;
        input = loop_input;
        if end.is_some() {
            break;
        }
    }
    Ok((input, hints))
}

#[derive(Debug, PartialEq)]
pub struct Hint {
    pub color: &'static str,
    pub count: usize,
}

impl Hint {
    pub fn is_valid(&self) -> bool {
        match self.color {
            "blue" => self.count <= 14,
            "green" => self.count <= 13,
            "red" => self.count <= 12,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: usize,
    pub hint: Vec<Vec<Hint>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_hint() {
        let input = ":1 blue,";
        let hint = parse_hint(input).unwrap();
        let expected = Hint {
            color: "blue",
            count: 1,
        };
        assert_eq!(hint.1, expected);
    }

    #[test]
    fn could_parse_hints() {
        let input = ":1 blue, 2 green; 3 green";
        let hint = parse_hints(input).unwrap();
        let expected = vec![
            Hint {
                color: "blue",
                count: 1,
            },
            Hint {
                color: "green",
                count: 2,
            },
        ];
        assert_eq!(hint.1, expected);
    }

    #[test]
    fn could_parse_game() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let valve = parse_game(input).unwrap();
        let expected = Game {
            id: 2,
            hint: vec![
                vec![
                    Hint {
                        color: "blue",
                        count: 1,
                    },
                    Hint {
                        color: "green",
                        count: 2,
                    },
                ],
                vec![
                    Hint {
                        color: "green",
                        count: 3,
                    },
                    Hint {
                        color: "blue",
                        count: 4,
                    },
                    Hint {
                        color: "red",
                        count: 1,
                    },
                ],
                vec![
                    Hint {
                        color: "green",
                        count: 1,
                    },
                    Hint {
                        color: "blue",
                        count: 1,
                    },
                ],
            ],
        };
        assert_eq!(valve, expected);
    }

    #[test]
    fn tmp() {
        use nom::bytes::complete::tag;
        use nom::multi::many1;

        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many1(tag("abc"))(s)
        }

        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abc"), Ok(("", vec!["abc"])));
    }
}
