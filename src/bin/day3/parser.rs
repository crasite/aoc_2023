use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while, take_while1},
    character::complete::{none_of, one_of},
    combinator::{eof, not, opt, verify},
    IResult, Parser,
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
    while opt(eof)(input)?.1.is_none() {
        let (new_input, token) = parse_token(input)?;
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
        let (new_input, width) = take_while(|c: char| c == '.')(new_input)?;
        x += width.len() as u32;
        input = new_input;
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
        let expected_symbol = vec![Symbol {
            name: "*",
            x: 4,
            y: y,
        }];
        let expected_number = vec![
            Number {
                value: 617,
                width: 3,
                x: 1,
                y: y,
            },
            Number {
                value: 33,
                width: 2,
                x: 10,
                y: y,
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
}
