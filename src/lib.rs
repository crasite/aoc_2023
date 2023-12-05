use nom::{
    character::complete::space0, error::ParseError, sequence::delimited, AsChar, IResult,
    InputTakeAtPosition, Parser,
};

pub fn space0_surrounded<I, O, E: ParseError<I>, F>(first: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition,
    F: Parser<I, O, E>,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(space0, first, space0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::VerboseError;

    #[test]
    fn space0_surrounded_test() {
        let input = "  123    ";
        let (_, output) =
            space0_surrounded::<_, _, VerboseError<&str>, _>(nom::character::complete::u32)(input)
                .unwrap();
        assert_eq!(output, 123);
    }
}
