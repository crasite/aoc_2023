use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::alpha0,
    combinator::{map, map_res, opt},
    IResult,
};

fn parse_number(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn parse_2nd_part(input: &str) -> IResult<&str, u32> {
    let (input, _) = alpha0(input)?;
    let (input, v) = map_res(take(1usize), parse_number)(input)?;
    Ok((input, v))
}

pub fn parse(input: &str) -> IResult<&str, u32> {
    let (input, _) = alpha0(input)?;
    let (mut remaining, initial) = map_res(take(1usize), parse_number)(input)?;
    let mut last = None;
    while let Ok((input, v)) = parse_2nd_part(remaining) {
        last = Some(v);
        remaining = input;
    }
    if let Some(last) = last {
        Ok((input, initial * 10 + last))
    } else {
        Ok((input, initial * 10 + initial))
    }
}

pub fn parse_text_number(input: &str) -> IResult<&str, u32> {
    let (input, v) = alt((
        map(tag("one"), |_| 1),
        map(tag("two"), |_| 2),
        map(tag("three"), |_| 3),
        map(tag("four"), |_| 4),
        map(tag("five"), |_| 5),
        map(tag("six"), |_| 6),
        map(tag("seven"), |_| 7),
        map(tag("eight"), |_| 8),
        map(tag("nine"), |_| 9),
    ))(input)?;
    Ok((input, v))
}

pub fn parse2(input: &str) -> IResult<&str, u32> {
    let mut remaining = input;
    let mut parse_result = alt((parse_text_number, map_res(take(1usize), parse_number)))(remaining);
    while let Err(_) = parse_result {
        (remaining, _) = take(1usize)(remaining)?;
        parse_result = alt((parse_text_number, map_res(take(1usize), parse_number)))(remaining);
    }
    let (_, initial) = parse_result?;
    let (mut remaining, _) = opt(take(1usize))(remaining)?; 
    let mut last = None;
    while remaining.len() > 0 {
        let tmp_result = alt((parse_text_number, map_res(take(1usize), parse_number)))(remaining);
        if let Ok((_, v)) = tmp_result {
            last = Some(v);
        }
        (remaining, _) = opt(take(1usize))(remaining)?;
    }
    if let Some(last) = last {
        Ok((remaining, initial * 10 + last))
    } else {
        Ok((remaining, initial * 10 + initial))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let input = "1abc2";
        assert_eq!(parse(input).unwrap().1, 12);
        let input = "pqr3stu8vwx";
        assert_eq!(parse(input).unwrap().1, 38);
        let input = "a1b2c3d4e5f";
        assert_eq!(parse(input).unwrap().1, 15);
        let input = "treb7uchet";
        assert_eq!(parse(input).unwrap().1, 77);
    }

    #[test]
    fn parser_2_test() {
        /*
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        */
        let input = "two1nine";
        assert_eq!(parse2(input).unwrap().1, 29);
        let input = "eightwothree";
        assert_eq!(parse2(input).unwrap().1, 83);
        let input = "abcone2threexyz";
        assert_eq!(parse2(input).unwrap().1, 13);
        let input = "xtwone3four";
        assert_eq!(parse2(input).unwrap().1, 24);
        let input = "4nineeightseven2";
        assert_eq!(parse2(input).unwrap().1, 42);
        let input = "zoneight234";
        assert_eq!(parse2(input).unwrap().1, 14);
        let input = "7pqrstsixteen";
        assert_eq!(parse2(input).unwrap().1, 76);
        let input = "sevent";
        assert_eq!(parse2(input).unwrap().1, 77);
        let input = "84";
        assert_eq!(parse2(input).unwrap().1, 84);
        let input = "eighthree";
        assert_eq!(parse2(input).unwrap().1, 83);
        let input = "sevenine";
        assert_eq!(parse2(input).unwrap().1, 79);
    }
}
