pub fn parse_reports(puzzle_input: &str) -> Vec<Vec<i32>> {
    puzzle_input.lines().map(|line| parse_report(line)).collect()
}

fn parse_report(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|level| level.parse::<i32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_report, parse_reports};

    #[test]
    fn parse_report_should_return_vec_of_i32() {
        assert_eq!(vec![7,6,4,2,1], parse_report("7 6 4 2 1"));
    }

const TEST_DATA: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn parse_reports_should_parse_test_data() {
        assert_eq!(
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ],
            parse_reports(TEST_DATA)
        )
    }
}
