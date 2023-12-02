use anyhow::Result;

mod parser;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let mut valves = vec![];
    for line in input.lines() {
        let valve = parser::parse_valve(line)?;
        valves.push(valve);
    }
    let functioning_valves = valves
        .iter()
        .filter(|v| v.rate > 0)
        .collect::<Vec<&parser::Valve>>();
    println!("{:?}", functioning_valves.len());
    Ok(())
}
