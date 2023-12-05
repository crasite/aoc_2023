use parser::SeedRange;
use rayon::prelude::*;

mod parser;

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
    println!("Part 2: {}", solve_part_2_brute_force(input));
}

fn solve_part_1(input: &'static str) -> i64 {
    let (seeds, maps) = parser::parse_input(input).unwrap().1;
    let mut min_value = i64::MAX;
    for seed in seeds {
        let mut src = seed;
        for map in &maps {
            src = map.get_dest(src)
        }
        if src < min_value {
            min_value = src;
        }
    }
    min_value
}

fn solve_part_2(input: &'static str) -> i64 {
    let (seeds, maps) = parser::parse_input(input).unwrap().1;
    let mut seed_ranges = SeedRange::new_from_vec(&seeds);
    let mut min_value = i64::MAX;
    for map in maps {
        let mut new_seed_ranges = vec![];
        for seed_range in &seed_ranges {
            new_seed_ranges.append(&mut map.get_dest_from_range(seed_range));
        }
        seed_ranges = new_seed_ranges;
    }
    for range in seed_ranges {
        if range.from < min_value {
            min_value = range.from;
        }
    }
    min_value
}

fn solve_part_2_brute_force(input: &'static str) -> i64 {
    let (seeds, maps) = parser::parse_input(input).unwrap().1;
    let seed_ranges = SeedRange::new_from_vec(&seeds);
    let mut min_value = i64::MAX;
    for range in seed_ranges {
        let new_low = (range.from..=range.to).into_par_iter().fold(
            || i64::MAX,
            |acc, seed| {
                let mut src = seed;
                for map in &maps {
                    src = map.get_dest(src)
                }
                if src < acc {
                    src
                } else {
                    acc
                }
            },
        ).min().unwrap();
        if new_low < min_value {
            min_value = new_low;
        }
    }
    min_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part_1() {
        let input = include_str!("./sample.txt");
        let output = 35;
        assert_eq!(solve_part_1(input), output);
    }

    #[test]
    fn could_solve_part_2() {
        let input = include_str!("./sample.txt");
        let output = 46;
        assert_eq!(solve_part_2(input), output);
    }
}
