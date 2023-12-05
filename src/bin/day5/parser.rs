use nom::{bytes::complete::{tag, take_until}, character::complete, multi::separated_list1, IResult, sequence::terminated};

#[derive(Debug, PartialEq)]
pub struct Entry {
    diff: i64,
    from: i64,
    end: i64,
}
#[derive(Debug, PartialEq)]
pub struct Map {
    from: &'static str,
    to: &'static str,
    entries: Vec<Entry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SeedRange {
    pub from: i64,
    pub to: i64,
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
            diff: dest-source,
            from: source,
            end: source+range-1,
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
        for entry in &self.entries {
            if source >= entry.from && source <= entry.end{
                rs = Some(source+entry.diff);
                break;
            }
        }
        match rs {
            Some(x) => x,
            None => source,
        }
    }

    pub fn get_dest_from_range(&self, source: &SeedRange) -> Vec<SeedRange> {
        let mut unprocessed = vec![source.clone()];
        let mut processed = vec![];
        for entry in &self.entries {
            let mut new_unprocessed = vec![];
            for range in &unprocessed {
                let mut new_processed = None;
                if range.from < entry.from && range.to >= entry.from {
                    new_unprocessed.push(SeedRange::new(range.from, entry.from-1));
                    if range.to > entry.end {
                        new_processed =Some(SeedRange::new(entry.from, entry.end));
                        new_unprocessed.push(SeedRange::new(entry.end+1, range.to));
                    } else {
                        new_processed = Some(SeedRange::new(entry.from, range.to));
                    } 
                } else if range.from >= entry.from && range.from <= entry.end {
                    if range.to > entry.end {
                        new_processed = Some(SeedRange::new(range.from, entry.end));
                        new_unprocessed.push(SeedRange::new(entry.end+1, range.to));
                    } else {
                        new_processed = Some(SeedRange::new(range.from, range.to));
                    }
                } else {
                    new_unprocessed.push(range.clone());
                }
                new_processed = new_processed.map(|x| SeedRange::new(x.from+entry.diff, x.to+entry.diff));
                if let Some(x) = new_processed {
                    processed.push(x);
                }
            }
            unprocessed = new_unprocessed;
        }
        processed.append(&mut unprocessed);
        processed
    }
}

impl SeedRange {
    pub fn new(from: i64, to: i64) -> Self {
        Self { from, to }
    }

    pub fn new_from_vec(value: &[i64]) -> Vec<Self> {
        let mut rs = Vec::new();
        for i in 0..value.len()/2 {
            let j = i*2;
            rs.push(Self::new(value[j], value[j]+value[j+1]-1));
        }
        rs

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
                    from: 15,
                    end: 51,
                    diff: -15
                },
                Entry {
                    from: 52,
                    end: 53,
                    diff: -15
                },
                Entry {
                    from: 0,
                    end: 14,
                    diff: 39
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
                    from: 15,
                    end: 52,
                    diff: -15
                },
                Entry {
                    from: 52,
                    end: 54,
                    diff: -15
                },
                Entry {
                    from: 0,
                    end: 15,
                    diff: 39
                },
            ],
        };
        let output = 53;
        assert_eq!(map.get_dest(input),output);
    }

    #[test]
    fn could_get_destination_from_range() {
        let seed_range = SeedRange::new_from_vec(&[1, 15, 16,33]);
        let map = Map {
            from: "soil",
            to: "fertilizer",
            entries: vec![
                Entry {
                    from: 15,
                    end: 52,
                    diff: -14
                },
                Entry {
                    from: 53,
                    end: 54,
                    diff: -15
                },
                Entry {
                    from: 1,
                    end: 14,
                    diff: 39
                },
            ],
        };
        let output = vec![SeedRange::new(1,1),SeedRange::new(40,53), SeedRange::new(2,34)];
        let mut rs = vec![];
        for range in &seed_range {
            rs.append(&mut map.get_dest_from_range(range));
        }
        assert_eq!(rs,output);
    }
}
