mod parser;

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
        .map(|report| is_safe_report(report))
        .filter(|is_safe| *is_safe)
        .count()
}

enum Status {
    Increasing,
    Decreasing,
}

fn is_safe_report(report: &[i32]) -> bool {
    let mut level_iterator = report.iter().peekable();
    let mut status = None;

    while let Some(level) = level_iterator.next() {
        if let Some(&next_level) = level_iterator.peek() {
            if !has_max_distance_of_three(level, next_level) {
                return false;
            }

            match status {
                None => {
                    if level < next_level {
                        status = Some(Status::Increasing);
                    } else if level > next_level {
                        status = Some(Status::Decreasing);
                    } else {
                        return false;
                    }
                }
                Some(Status::Increasing) => if level >= next_level {
                    return false;
                }
                Some(Status::Decreasing) => if level <= next_level {
                    return false;
                }
            }
        }
    }

    true
}

fn has_max_distance_of_three(a: &i32, b: &i32) -> bool {
    (a - b).abs() <= 3
}

#[cfg(test)]
mod tests {
    use crate::{count_safe_reports, is_safe_report};

    #[test]
    fn is_safe_report_should_return_true_for_7_6_4_2_1() {
        assert!(is_safe_report(&vec![7, 6, 4, 2, 1]));
    }

    #[test]
    fn is_safe_report_should_return_false_for_1_2_7_8_9() {
        assert!(!is_safe_report(&vec![1, 2, 7, 8, 9]));
    }

    #[test]
    fn is_safe_report_should_return_false_for_9_7_6_2_1() {
        assert!(!is_safe_report(&vec![9, 7, 6, 2, 1]));
    }

    #[test]
    fn is_safe_report_should_return_false_for_1_3_2_4_5() {
        assert!(!is_safe_report(&vec![1, 3, 2, 4, 5]));
    }

    #[test]
    fn is_safe_report_should_return_false_for_8_6_4_4_1() {
        assert!(!is_safe_report(&vec![8, 6, 4, 4, 1]));
    }

    #[test]
    fn is_safe_report_should_return_true_for_1_3_6_7_9() {
        assert!(is_safe_report(&vec![1, 3, 6, 7, 9]));
    }

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