use aoc::space0_surrounded;
use nom::{
    bytes::complete::{tag, take_till1},
    character::complete,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    pub fn winning_hold_duration(&self) -> (u64, u64) {
        let mut first_value = None;
        let mut last_value = None;
        for i in 1..self.time {
            let distance = i * (self.time - i);
            if distance > self.distance {
                first_value = Some(i);
                break;
            }
        }
        for i in (1..self.time).rev() {
            let distance = i * (self.time - i);
            if distance > self.distance {
                last_value = Some(i);
                break;
            }
        }
        (first_value.unwrap(), last_value.unwrap())
    }
}

pub fn parse_game(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, time_list) = many1(space0_surrounded(complete::u64))(input)?;
    let (input, _) = tag("\nDistance:")(input)?;
    let (input, distance_list) = many1(space0_surrounded(complete::u64))(input)?;
    let mut rs = vec![];
    for i in 0..time_list.len() {
        rs.push(Race {
            time: time_list[i],
            distance: distance_list[i],
        })
    }
    Ok((input, rs))
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    let (input, _) = take_till1(|c: char| c.is_ascii_digit())(input)?;
    let (input, number) = take_till1(|c: char| c == '\n')(input)?;
    let number = number.replace(' ', "");
    let number = number.parse::<u64>().unwrap();
    Ok((input, number))
}

pub fn parse_game2(input: &str) -> IResult<&str, Race> {
    let (input, value) = separated_list1(complete::newline, parse_number)(input)?;
    Ok((
        input,
        Race {
            time: value[0],
            distance: value[1],
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_race() {
        let input = include_str!("./sample.txt");
        let output = vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ];
        assert_eq!(parse_game(input).unwrap().1, output);
    }

    #[test]
    fn could_parse_race2() {
        let input = include_str!("./sample.txt");
        let output = Race {
            time: 71530,
            distance: 940200,
        };
        assert_eq!(parse_game2(input).unwrap().1, output);
    }

    #[test]
    fn could_calculate_winning_duration() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        let output = (2, 5);
        assert_eq!(race.winning_hold_duration(), output);
    }
}
