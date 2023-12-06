use aoc::space0_surrounded;
use nom::{IResult, bytes::complete::tag, multi::{separated_list1, self, many1}, character::complete};

#[derive(Debug, PartialEq)]
pub struct Race {
    time: i32,
    distance: i32,
}
impl Race {
    pub fn winning_hold_duration(&self) -> (i32, i32) {
        let mut first_value = None;
        let mut last_value = None;
        for i in 1..self.time {
            let distance = i*(self.time-i);
            if distance > self.distance {
                if first_value.is_none() {
                    first_value = Some(i);
                } else {
                    last_value = Some(i);
                }
            }
        }
        (first_value.unwrap(), last_value.unwrap())
    }
}

pub fn parse_game(input: &str) -> IResult<&str, Vec<Race>>{
    let (input, _) = tag("Time:")(input)?;
    let (input, time_list) = many1(space0_surrounded(complete::i32))(input)?;
    let (input, _) = tag("\nDistance:")(input)?;
    let (input, distance_list) = many1(space0_surrounded(complete::i32))(input)?;
    let mut rs = vec![];
    for i in 0..time_list.len() {
        rs.push(Race{
            time: time_list[i],
            distance: distance_list[i],
        })
    }
    Ok((input, rs))
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
    fn could_calculate_winning_duration() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        let output = (2,5);
        assert_eq!(race.winning_hold_duration(), output);
    }
}
