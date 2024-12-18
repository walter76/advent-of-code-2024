use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestEquation {
    result: i32,
    operands: Vec<i32>,
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
    use crate::TestEquation;

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
}