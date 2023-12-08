use nom::{
    bytes::complete::{tag, take, take_till1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    directions: Vec<char>,
    pub index: usize,
}
impl Instruction {
    pub fn next(&mut self) -> char {
        let direction = self.directions[self.index];
        self.index = (self.index + 1) % self.directions.len();
        direction
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: &'static str,
    pub left: &'static str,
    pub right: &'static str,
}

fn parse_instructions(input: &'static str) -> IResult<&str, Instruction> {
    let (input, instructions) = take_till1(|c| c == '\n')(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let directions = instructions.chars().collect();
    Ok((
        input,
        Instruction {
            directions,
            index: 0,
        },
    ))
}

fn parse_node(input: &'static str) -> IResult<&str, Node> {
    let (input, name) = take(3usize)(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = take(3usize)(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = take(3usize)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node { name, left, right }))
}

pub fn parse(input: &'static str) -> IResult<&str, (Instruction, Vec<Node>)> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, nodes) = separated_list1(tag("\n"), parse_node)(input)?;
    Ok((input, (instructions, nodes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let expected = (
            Instruction {
                directions: vec!['L', 'L', 'R'],
                index: 0,
            },
            vec![
                Node {
                    name: "AAA",
                    left: "BBB",
                    right: "BBB",
                },
                Node {
                    name: "BBB",
                    left: "AAA",
                    right: "ZZZ",
                },
                Node {
                    name: "ZZZ",
                    left: "ZZZ",
                    right: "ZZZ",
                },
            ],
        );

        assert_eq!(parse(input).unwrap().1, expected);
    }

    #[test]
    fn could_parse_node() {
        let input = "DBQ = (RTP, NBX)";
        let expected = Node {
            name: "DBQ",
            left: "RTP",
            right: "NBX",
        };
        assert_eq!(parse_node(input).unwrap().1, expected);
    }
}
