use nom::{bytes::complete::{tag, take_until}, character::complete, multi::separated_list1, IResult, sequence::terminated};

#[derive(Debug, PartialEq)]
pub struct Entry {
    dest: i64,
    source: i64,
    range: i64,

}
#[derive(Debug, PartialEq)]
pub struct Map {
    from: &'static str,
    to: &'static str,
    entries: Vec<Entry>,
}

fn parse_seed(input: &'static str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("seeds: ")(input)?;
    separated_list1(complete::space1, complete::i64)(input)
}

fn parse_entry(input: &'static str) -> IResult<&str, Entry> {
    let (input, dest) = complete::i64(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, source) = complete::i64(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, range) = complete::i64(input)?;
    Ok((
        input,
        Entry {
            dest,
            source,
            range,
        },
    ))
}

fn parse_map(input: &'static str) -> IResult<&str, Map> {
    let (input, from) = take_until("-")(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, to) = take_until(" ")(input)?;
    let (input, _) = terminated(take_until("\n"), tag("\n"))(input)?;
    let (input, entries) = separated_list1(complete::newline, parse_entry)(input)?;
    Ok((
        input,
        Map {
            from,
            to,
            entries,
        },
    ))
}


pub fn parse_input(input: &'static str) -> IResult<&str, (Vec<i64>, Vec<Map>)> {
    let (input, seeds) = parse_seed(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, maps) = separated_list1(tag("\n\n"), parse_map)(input)?;
    Ok((input, (seeds, maps)))
}


impl Map {
    pub fn get_dest(&self, source: i64) -> i64 {
        let mut rs = None;
        for entrie in &self.entries {
            if source >= entrie.source && source < entrie.source + entrie.range {
                rs = Some(entrie.dest + source - entrie.source);
                break;
            }
        }
        match rs {
            Some(x) => x,
            None => source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_seed() {
        let input = "seeds: 79 14 55 13";
        let output = vec![79, 14, 55, 13];
        assert_eq!(parse_seed(input).unwrap().1, output);
    }

    #[test]
    fn could_parse_map() {
        let input = r#"soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"#;
        let output = Map {
            from: "soil",
            to: "fertilizer",
            entries: vec![
                Entry {
                    dest: 0,
                    source: 15,
                    range: 37,
                },
                Entry {
                    dest: 37,
                    source: 52,
                    range: 2,
                },
                Entry {
                    dest: 39,
                    source: 0,
                    range: 15,
                },
            ],
        };
        assert_eq!(parse_map(input).unwrap().1, output);
    }

    #[test]
    fn could_get_destination() {
        let input = 14;
        let map = Map {
            from: "soil",
            to: "fertilizer",
            entries: vec![
                Entry {
                    dest: 0,
                    source: 15,
                    range: 37,
                },
                Entry {
                    dest: 37,
                    source: 52,
                    range: 2,
                },
                Entry {
                    dest: 39,
                    source: 0,
                    range: 15,
                },
            ],
        };
        let output = 53;
        assert_eq!(map.get_dest(input),output);
    }
}
