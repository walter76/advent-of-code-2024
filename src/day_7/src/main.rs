use std::time::Instant;

use anyhow::Result;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 7)?;
    
    let start = Instant::now();
    
    let test_equations = parse_calibration_equations(&puzzle_input);

    let duration = start.elapsed();

    println!("Parsing took {:?}", duration);

    let start = Instant::now();

    let sum = sum_of_valid_test_equations(&test_equations);

    let duration = start.elapsed();

    println!("The total calibration result is: {} (took {:?})", sum, duration);

    let start = Instant::now();

    let sum = sum_of_valid_test_equations_recursive(&test_equations);

    let duration = start.elapsed();

    println!("The total calibration result is: {} (recursive variant took {:?})", sum, duration);

    Ok(())
}

fn sum_of_valid_test_equations(test_equations: &[TestEquation]) -> i64 {
    test_equations.iter()
        .filter(|test_equation| test_equation.is_valid())
        .map(|test_equation| test_equation.result)
        .sum()
}

fn sum_of_valid_test_equations_recursive(test_equations: &[TestEquation]) -> i64 {
    test_equations.iter()
        .filter(|test_equation| test_equation.is_valid_recursive())
        .map(|test_equation| test_equation.result)
        .sum()
}

fn parse_calibration_equations(input: &str) -> Vec<TestEquation> {
    input.lines().map(|line| TestEquation::from(line)).collect()
}

// XXX: Operators are always evaluated left-to-right, not according to precedence rules.

// - only possibility is to try all combinations
// - optimizations:
//   - always left-to-right, no precendence
//   - do multiplication first because that yields higher results and enables
//     earlier backtracking

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestEquation {
    result: i64,
    operands: Vec<i64>,
}

impl TestEquation {
    pub fn is_valid(&self) -> bool {
        let n = self.operands.len();

        if n == 0 {
            return true;
        }

        if n == 1 {
            return self.operands[0] == self.result;
        }

        // there are 2^(n-1) ways to place +/* between n numbers
        let total_combinations = 1 << (n - 1);

        for mask in 0 .. total_combinations {
            // start with the first number
            let mut value = self.operands[0];

            // for each "operator slot"
            for i in 0 .. (n - 1) {
                // extract the bit i from mask
                let bit = (mask >> i) & 1;

                if bit == 0 {
                    // 0 means '+'
                    value += self.operands[i + 1];
                } else {
                    // 1 means '*'
                    value *= self.operands[i + 1];
                }
            }

            if value == self.result {
                return true;
            }
        }

        false
    }

    pub fn is_valid_recursive(&self) -> bool {
        let n = self.operands.len();

        if n == 0 {
            return true;
        }

        if n == 1 {
            return self.operands[0] == self.result;
        }

        let mut operands = self.operands.clone();

        operands.reverse();

        let results = self.all_combinations_recursive(&operands);

        results.iter().any(|(_, result)| *result == self.result)
    }

    fn all_combinations_recursive(&self, operands: &[i64]) -> Vec<(String, i64)> {
        // base case: if there's only one number, there's exactly
        // one expression: "num" -> its value.
        if operands.len() == 1 {
            return vec![(operands[0].to_string(), operands[0])];
        }

        // split: first number and the rest of the slice
        let first = operands[0];
        let tail = &operands[1 ..];

        // recursively get all combinations for the tail
        let sub_combinations = self.all_combinations_recursive(tail);

        let mut results = vec![];

        // for every sub-expression, combine with the first number
        for (expr, val) in sub_combinations {
            // using '+'
            let expr_plus = format!("{}+{}", first, expr);
            let val_plus = first + val;
            results.push((expr_plus, val_plus));

            // using '*'
            let expr_mul = format!("{}*{}", first, expr);
            let val_mul = first * val;
            results.push((expr_mul, val_mul));
        }

        results
    }
}

impl From<&str> for TestEquation {
    fn from(value: &str) -> Self {
        let pos_colon = value.chars().position(|c| c == ':')
            .expect("could not find a colon in the string");

        let result = value[ .. pos_colon].parse()
            .expect("could not parse the equation result");

        let operands = value[ pos_colon + 1 .. ]
            .split_whitespace()
            .map(|o| o.parse().unwrap())
            .collect();

        Self {
            result,
            operands,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_calibration_equations, sum_of_valid_test_equations, sum_of_valid_test_equations_recursive, TestEquation};

    const FIRST_EXAMPLE: &str = "190: 10 19";

    #[test]
    fn from_str_should_parse_first_example() {
        assert_eq!(
            TestEquation {
                result: 190,
                operands: vec![10, 19],
            },
            TestEquation::from(FIRST_EXAMPLE)
        );
    }

    const SECOND_EXAMPLE: &str = "3267: 81 40 27";

    #[test]
    fn from_str_should_parse_second_example() {
        assert_eq!(
            TestEquation {
                result: 3267,
                operands: vec![81, 40, 27],
            },
            TestEquation::from(SECOND_EXAMPLE)
        );
    }

    #[test]
    fn is_valid_should_return_true_for_first_example() {
        let test_equation = TestEquation::from(FIRST_EXAMPLE);
        assert!(test_equation.is_valid());
    }

    #[test]
    fn is_valid_should_return_true_for_second_example() {
        let test_equation = TestEquation::from(SECOND_EXAMPLE);
        assert!(test_equation.is_valid());
    }

    const THIRD_EXAMPLE: &str = "83: 17 5";

    #[test]
    fn is_valid_should_return_false_for_third_example() {
        let test_equation = TestEquation::from(THIRD_EXAMPLE);
        assert!(!test_equation.is_valid());
    }

    const EXAMPLE_DATA: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn sum_of_valid_test_equations_should_return_3749_for_example_data() {
        let test_equations = parse_calibration_equations(EXAMPLE_DATA);
        assert_eq!(3749, sum_of_valid_test_equations(&test_equations));
    }

    #[test]
    fn all_combinations_recursive_should_return_two_results_for_first_example() {
        let test_equation = TestEquation::from(FIRST_EXAMPLE);
        let mut operands = test_equation.operands.clone();

        operands.reverse();

        assert_eq!(
            vec![
                (String::from("19+10"), 29),
                (String::from("19*10"), 190),
            ],
            test_equation.all_combinations_recursive(&operands));
    }

    #[test]
    fn all_combinations_recursive_should_return_four_results_for_second_example() {
        let test_equation = TestEquation::from(SECOND_EXAMPLE);
        let mut operands = test_equation.operands.clone();

        operands.reverse();

        assert_eq!(
            vec![
                (String::from("27+40+81"), 148),
                (String::from("27*40+81"), 3267),
                (String::from("27+40*81"), 3267),
                (String::from("27*40*81"), 87480),
            ],
            test_equation.all_combinations_recursive(&operands));
    }

    #[test]
    fn all_combinations_recursive_should_return_two_results_for_third_example() {
        let test_equation = TestEquation::from(THIRD_EXAMPLE);
        let mut operands = test_equation.operands.clone();

        operands.reverse();

        assert_eq!(
            vec![
                (String::from("5+17"), 22),
                (String::from("5*17"), 85),
            ],
            test_equation.all_combinations_recursive(&operands));
    }

    #[test]
    fn is_valid_rescursive_should_return_true_for_first_example() {
        let test_equation = TestEquation::from(FIRST_EXAMPLE);
        assert!(test_equation.is_valid_recursive());
    }

    #[test]
    fn is_valid_recursive_should_return_true_for_second_example() {
        let test_equation = TestEquation::from(SECOND_EXAMPLE);
        assert!(test_equation.is_valid_recursive());
    }

    #[test]
    fn is_valid_recursive_should_return_false_for_third_example() {
        let test_equation = TestEquation::from(THIRD_EXAMPLE);
        assert!(!test_equation.is_valid_recursive());
    }

    #[test]
    fn sum_of_valid_test_recursive_equations_should_return_3749_for_example_data() {
        let test_equations = parse_calibration_equations(EXAMPLE_DATA);
        assert_eq!(3749, sum_of_valid_test_equations_recursive(&test_equations));
    }
}