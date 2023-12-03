use nom::{IResult, character::complete::i32 as parse_i32, bytes::complete::tag};

use crate::model::DropPart;


pub fn parse_droplet(input: &'static str) -> IResult<&str, DropPart> {
    let (input, x) = parse_i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = parse_i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = parse_i32(input)?;
    Ok((input, DropPart::new(x,y,z)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_part() {
        let input = "3,2,5";
        let expected = DropPart::new(3,2,5);
        assert_eq!(parse_droplet(input).unwrap().1, expected);
    }
}
