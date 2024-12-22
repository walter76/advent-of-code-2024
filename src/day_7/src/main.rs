use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

fn sum_of_valid_test_equations(test_equations: &[TestEquation]) -> i32 {
    test_equations.iter()
        .filter(|test_equation| test_equation.is_valid())
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
    result: i32,
    operands: Vec<i32>,
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
    use crate::{parse_calibration_equations, sum_of_valid_test_equations, TestEquation};

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
}