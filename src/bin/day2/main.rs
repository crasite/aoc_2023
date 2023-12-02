use anyhow::Result;

mod parser;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let result = solve_part_1(input)?;
    println!("Part 1: {}", result);
    let result = solve_part_2(input)?;
    println!("Part 2: {}", result);
    Ok(())
}

fn solve_part_1(input: &'static str) -> Result<usize> {
    let mut valid_sum = 0;
    'a: for line in input.lines() {
        let game = parser::parse_game(line)?;
        for hintset in game.hint {
            for hint in hintset {
                if !hint.is_valid() {
                    continue 'a;
                }
            }
        }
        valid_sum += game.id;
    }
    Ok(valid_sum)
}

fn solve_part_2(input: &'static str) -> Result<usize> {
    let mut valid_sum = 0;
    for line in input.lines() {
        let game = parser::parse_game(line)?;
        let mut max_green = 0;
        let mut max_red = 0;
        let mut max_blue = 0;
        for hintset in game.hint {
            for hint in hintset {
                match hint.color {
                    "blue" => max_blue = max_blue.max(hint.count),
                    "green" => max_green = max_green.max(hint.count),
                    "red" => max_red = max_red.max(hint.count),
                    _ => (),
                }
            }
        }
        valid_sum += max_blue * max_green * max_red;
    }
    Ok(valid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_solve_part_1() {
        let input = include_str!("sample.txt");
        let result = solve_part_1(input).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn could_solve_part_2() {
        let input = include_str!("sample.txt");
        let result = solve_part_2(input).unwrap();
        assert_eq!(result, 2286);
    }
}
