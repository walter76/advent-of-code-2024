mod parser;
mod report_checker;

use anyhow::Result;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 2)?;

    let reports = parser::parse_reports(&puzzle_input);
    let number_of_safe_reports = report_checker::count_safe_reports(reports);

    println!("Number of safe reports: {}", number_of_safe_reports);

    Ok(())
}
