use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take, take_till1},
    character::complete::{digit1, multispace0},
    combinator::{map_res, opt},
    multi::many1,
    sequence::terminated,
    IResult,
};

fn parse_valve_name(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = take(2usize)(input)?;
    Ok((input, name))
}

fn parse_valve_rate(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, rate) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    Ok((input, rate))
}

fn parse_valve_connection(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, name) = terminated(take(2usize), opt(tag(",")))(input)?;
    Ok((input, name))
}

fn valve_parser(input: &'static str) -> IResult<&str, Valve> {
    let (input, name) = parse_valve_name(input)?;
    let (input, rate) = parse_valve_rate(input)?;
    let (input, _) = take_till1(|c: char| c.is_uppercase())(input)?;
    let (input, connections) = many1(parse_valve_connection)(input)?;
    let valve = Valve {
        name,
        rate,
        connections,
        is_open: false,
    };
    Ok((input, valve))
}

pub fn parse_valve(input: &'static str) -> Result<Valve> {
    let (_, valve) = valve_parser(input)?;
    Ok(valve)
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub struct Valve {
    name: &'static str,
    pub rate: u32,
    connections: Vec<&'static str>,
    is_open: bool,
}

impl Valve {
    pub fn get_connections<'a>(&self, valves: &'a [Valve]) -> Result<Vec<&'a Valve>> {
        let mut connections = Vec::new();
        for name in self.connections.iter() {
            let valve = valves
                .iter()
                .find(|v| v.name == *name)
                .ok_or(anyhow!("Valve {} not found", name))?;
            connections.push(valve);
        }
        Ok(connections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_valve() {
        let input = "Valve BB has flow rate=13; tunnels lead to valves CC, AA";
        let valve = parse_valve(input).unwrap();
        let expected = Valve {
            name: "BB",
            rate: 13,
            connections: vec!["CC", "AA"],
            is_open: false,
        };
        assert_eq!(valve, expected);
    }
}
