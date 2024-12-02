mod parser;
mod report_checker;

use anyhow::Result;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 2)?;

    let reports = parser::parse_reports(&puzzle_input);
    let number_of_safe_reports = count_safe_reports(&reports);

    println!("Number of safe reports: {}", number_of_safe_reports);

    let number_of_safe_reports = 
        count_safe_reports_with_problem_dampener_active(&reports);

    println!(
        "Number of safe reports (problem dampener active): {}",
        number_of_safe_reports);

    Ok(())
}

fn count_safe_reports_with_problem_dampener_active(reports: &Vec<Vec<i32>>)
    -> usize
{
    let mut number_of_safe_reports = 0;

    for report in reports.iter() {
        if report_checker::is_safe_report(report)
            || is_safe_with_problem_dampener(report)
        {
            number_of_safe_reports += 1;
        }
    }

    number_of_safe_reports
}

fn is_safe_with_problem_dampener(report: &Vec<i32>) -> bool {
    for i in 0 .. report.len() {
        let mut report_with_level_removed = report.clone();

        report_with_level_removed.remove(i);

        if report_checker::is_safe_report(&report_with_level_removed) {
            return true;
        }
    }

    false
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter()
        .map(|report| report_checker::is_safe_report(report))
        .filter(|is_safe| *is_safe)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{count_safe_reports, count_safe_reports_with_problem_dampener_active, is_safe_with_problem_dampener};

    #[test]
    fn count_safe_reports_should_return_2_for_test_data() {
        assert_eq!(
            2,
            count_safe_reports(
                &vec![
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

    #[test]
    fn is_safe_with_problem_dampener_should_return_false_for_1_2_7_8_9() {
        assert!(!is_safe_with_problem_dampener(&vec![1, 2, 7, 8, 9]));
    }

    #[test]
    fn is_safe_with_problem_dampener_should_return_false_for_9_7_6_2_1() {
        assert!(!is_safe_with_problem_dampener(&vec![9, 7, 6, 2, 1]));
    }

    #[test]
    fn is_safe_with_problem_dampener_should_return_true_for_1_3_2_4_5() {
        assert!(is_safe_with_problem_dampener(&vec![1, 3, 2, 4, 5]));
    }

    #[test]
    fn is_safe_with_problem_dampener_should_return_true_for_8_6_4_4_1() {
        assert!(is_safe_with_problem_dampener(&vec![8, 6, 4, 4, 1]));
    }

    #[test]
    fn count_safe_reports_with_problem_dampener_active_should_return_4_for_test_data() {
        assert_eq!(
            4,
            count_safe_reports_with_problem_dampener_active(
                &vec![
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
