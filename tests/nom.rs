use nom::{
    bytes::complete::{tag, take_while},
    character::complete::u32 as parse_u32,
    sequence::delimited,
    IResult, Parser,
};

fn take_number(input: &str) -> IResult<&str, u32> {
    let (input, number) = delimited(tag("("), take_while(|c: char| c.is_numeric()), tag(")"))
        .and_then(parse_u32)
        .parse(input)?;
    Ok((input, number))
}

#[test]
fn sample() {
    let input = "(123)";
    let (input, number) = take_number(input).unwrap();
    assert_eq!(number, 123);
}
