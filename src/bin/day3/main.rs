mod parser;
fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &'static str) -> u32 {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();
    let mut rs = 0;
    for (i,line) in input.lines().enumerate() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part1() {
        let input = include_str!("sample.txt");
        assert_eq!(4361, part1(input))
    }
}
