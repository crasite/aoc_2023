use nom::{
    branch::alt,
    bytes::complete::{take, take_while, take_while1},
    character::complete::one_of,
    combinator::not,
    IResult,
};

#[derive(Debug, PartialEq)]
enum Token {
    Symbol(&'static str),
    Number(&'static str),
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    name: &'static str,
    x: u32,
    y: u32,
}

impl Symbol {
    pub fn get_gear_ratio(&self, numbers: &[Number]) -> usize {
        if self.name != "*" {
            return 0;
        }
        let mut gears = vec![];
        'a: for number in numbers {
            for y in number.y.saturating_sub(1)..=number.y + 1 {
                if self.y != y {
                    continue;
                }
                for x in number.x.saturating_sub(1)..=number.x + number.width {
                    if self.x == x {
                        gears.push(number);
                        continue 'a;
                    }
                }
            }
        }
        if gears.len() == 2 {
            gears[0].value as usize * gears[1].value as usize
        } else {
            0
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: u32,
    width: u32,
    x: u32,
    y: u32,
}
impl Number {
    pub fn is_part(&self, symbol_list: &[Symbol]) -> bool {
        let mut is_part = false;
        'a: for symbol in symbol_list {
            for y in symbol.y.saturating_sub(1)..=symbol.y + 1 {
                if self.y != y {
                    continue;
                }
                for x in symbol.x.saturating_sub(1)..=symbol.x + 1 {
                    if self.x == x {
                        is_part = true;
                        break 'a;
                    }
                    if self.x <= x && self.x + self.width >= symbol.x {
                        is_part = true;
                        break 'a;
                    }
                }
            }
        }
        is_part
    }
}

fn parse_symbol(input: &'static str) -> IResult<&str, Token> {
    let (input, _) = not(one_of(".0123456789"))(input)?;
    let (input, symbol) = take(1usize)(input)?;
    Ok((input, Token::Symbol(symbol)))
}

fn parse_number(input: &'static str) -> IResult<&str, Token> {
    let (input, number) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    Ok((input, Token::Number(number)))
}

fn parse_token(input: &'static str) -> IResult<&str, Token> {
    let (input, token) = alt((parse_symbol, parse_number))(input)?;
    Ok((input, token))
}

pub fn parse_line(input: &'static str, y: u32) -> IResult<&str, (Vec<Symbol>, Vec<Number>)> {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();
    let mut x = 0;
    let (mut input, width) = take_while(|c: char| c == '.')(input)?;
    x += width.len() as u32;
    while !input.is_empty() {
        let token;
        (input, token) = parse_token(input)?;
        match token {
            Token::Symbol(name) => {
                symbols.push(Symbol { name, x, y });
                x += 1;
            }
            Token::Number(value) => {
                let width = value.len() as u32;
                numbers.push(Number {
                    value: value.parse().unwrap(),
                    width,
                    x,
                    y,
                });
                x += width;
            }
        }
        let width;
        (input, width) = take_while(|c: char| c == '.')(input)?;
        x += width.len() as u32;
    }
    Ok((input, (symbols, numbers)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_symbol() {
        let input = "$22.";
        let expected = Token::Symbol("$");
        assert_eq!(expected, parse_token(input).unwrap().1);
    }

    #[test]
    fn could_parse_number() {
        let input = "123.";
        let expected = Token::Number("123");
        assert_eq!(expected, parse_token(input).unwrap().1);
    }

    #[test]
    fn could_parse_line() {
        let y = 0;
        let input = ".617*.....33.";
        let expected_symbol = vec![Symbol { name: "*", x: 4, y }];
        let expected_number = vec![
            Number {
                value: 617,
                width: 3,
                x: 1,
                y,
            },
            Number {
                value: 33,
                width: 2,
                x: 10,
                y,
            },
        ];
        let (symbols, numbers) = parse_line(input, y).unwrap().1;
        assert_eq!(expected_symbol, symbols);
        assert_eq!(expected_number, numbers);
    }

    #[test]
    fn could_check_if_part() {
        let symbol = vec![Symbol {
            name: "*",
            x: 4,
            y: 0,
        }];
        let number = Number {
            value: 617,
            width: 3,
            x: 1,
            y: 0,
        };
        assert!(number.is_part(&symbol));
    }

    #[test]
    fn could_get_gear_ratio() {
        let symbol = Symbol {
            name: "*",
            x: 4,
            y: 0,
        };
        let numbers = vec![Number {
            value: 617,
            width: 3,
            x: 1,
            y: 0,
        }];
        assert_eq!(symbol.get_gear_ratio(&numbers), 0);
        let symbol = Symbol {
            name: "*",
            x: 4,
            y: 0,
        };
        let numbers = vec![
            Number {
                value: 467,
                width: 3,
                x: 1,
                y: 0,
            },
            Number {
                value: 35,
                width: 3,
                x: 1,
                y: 1,
            },
        ];
        assert_eq!(symbol.get_gear_ratio(&numbers), 16345);
    }
}
