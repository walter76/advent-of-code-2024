mod parser;
mod report_checker;

use anyhow::Result;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 2)?;

    let reports = parser::parse_reports(&puzzle_input);
    let number_of_safe_reports = count_safe_reports(reports);

    println!("Number of safe reports: {}", number_of_safe_reports);

    Ok(())
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> usize {
    reports.iter()
        .map(|report| report_checker::is_safe_report(report))
        .filter(|is_safe| *is_safe)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::count_safe_reports;

    #[test]
    fn count_safe_reports_should_return_2_for_test_data() {
        assert_eq!(
            2,
            count_safe_reports(
                vec![
                    vec![7, 6, 4, 2, 1],
                    vec![1, 2, 7, 8, 9],
                    vec![9, 7, 6, 2, 1],
                    vec![1, 3, 2, 4, 5],
                    vec![8, 6, 4, 4, 1],
                    vec![1, 3, 6, 7, 9],
                ],
            )
        );
    }
}
