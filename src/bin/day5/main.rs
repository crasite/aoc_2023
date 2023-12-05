use nom::bytes::complete::tag;


mod parser;

fn main(){
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
}

fn solve_part_1(input: &'static str) -> i64 {
    let (seeds,maps) = parser::parse_input(input).unwrap().1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part_1() {
        let input = include_str!("./sample.txt");
        let output = 35;
        assert_eq!(solve_part_1(input), output);
    }
}
