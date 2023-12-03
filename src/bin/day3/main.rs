mod parser;
fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &'static str) -> u32 {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();
    let mut rs = 0;
    for (i, line) in input.lines().enumerate() {
        let (_, (new_symbols, new_numbers)) = parser::parse_line(line, i as u32).unwrap();
        symbols.extend(new_symbols);
        numbers.extend(new_numbers);
    }
    for number in &numbers {
        if number.is_part(&symbols) {
            rs += number.value;
        }
    }
    rs
}

fn part2(input: &'static str) -> usize {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let (_, (new_symbols, new_numbers)) = parser::parse_line(line, i as u32).unwrap();
        symbols.extend(new_symbols);
        numbers.extend(new_numbers);
    }
    let rs = symbols
        .iter()
        .fold(0, |acc, s| acc + s.get_gear_ratio(&numbers));
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part1() {
        let input = include_str!("sample.txt");
        assert_eq!(4361, part1(input))
    }

    #[test]
    fn could_solve_part2() {
        let input = include_str!("sample.txt");
        assert_eq!(467835, part2(input))
    }
}
