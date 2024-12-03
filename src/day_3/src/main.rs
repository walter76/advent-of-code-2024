use regex::Regex;

fn main() {
    println!("Hello, world!");
}

fn sum_uncorrupted_mul_instructions(puzzle_input: &str) -> i32 {
    parse(puzzle_input)
        .unwrap()
        .iter()
        .map(|(num1, num2)| num1 * num2)
        .sum()
}

fn parse(puzzle_input: &str) -> Option<Vec<(i32, i32)>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    let result: Vec<(i32,i32)> = re.captures_iter(puzzle_input)
        .map(|cap| {
            let num1: i32 = cap[1].parse().expect("Failed to parse num1");
            let num2: i32 = cap[2].parse().expect("Failed to parse num1");
            (num1, num2)
        })
        .collect();

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, sum_uncorrupted_mul_instructions};

    #[test]
    fn parse_should_parse_regular_mul_instructions() {
        assert_eq!(Some(vec![(44,46)]), parse("mul(44,46)"));
        assert_eq!(Some(vec![(123,4)]), parse("mul(123,4)"));
    }

    #[test]
    fn parse_should_ignore_wrong_mul_instructions() {
        assert_eq!(None, parse("mul(4*"));
        assert_eq!(None, parse("mul(6,9!"));
        assert_eq!(None, parse("?(12,34)"));
        assert_eq!(None, parse("mul ( 2 , 4 )"));
    }

    const TEST_DATA: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn parse_should_parse_test_data() {
        assert_eq!(
            Some(vec![
                (2,4),
                (5,5),
                (11,8),
                (8,5),
            ]),
            parse(TEST_DATA)
        );
    }

    #[test]
    fn sum_uncorrupted_mul_instructions_should_return_161_for_test_data() {
        assert_eq!(161, sum_uncorrupted_mul_instructions(TEST_DATA));
    }
}