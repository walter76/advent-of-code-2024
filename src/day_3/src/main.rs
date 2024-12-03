mod instructions_parser;
mod mul_parser;

use anyhow::Result;
use instructions_parser::Instruction;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 3)?;

    let sum = sum_uncorrupted_mul_instructions(&puzzle_input);

    println!("The sum of all uncorrupted mul instructions in the input is: {}", sum);

    let sum = sum_mul(&puzzle_input);

    println!("The sum of all uncorrupted mul instructions with enabler in the input is: {}", sum);

    Ok(())
}

fn sum_uncorrupted_mul_instructions(puzzle_input: &str) -> i32 {
    mul_parser::parse(puzzle_input)
        .unwrap()
        .iter()
        .map(|(num1, num2)| num1 * num2)
        .sum()
}

fn sum_mul(puzzle_input: &str) -> i32 {
    let instructions = instructions_parser::parse(puzzle_input).unwrap();

    let mut sum = 0;
    let mut mul_enabled = true;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Do => mul_enabled = true,
            Instruction::Dont => mul_enabled = false,
            Instruction::Mul(num1, num2) => {
                if mul_enabled {
                    sum += num1 * num2
                }
            }
        }

    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{sum_mul, sum_uncorrupted_mul_instructions};

    const TEST_DATA: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn sum_uncorrupted_mul_instructions_should_return_161_for_test_data() {
        assert_eq!(161, sum_uncorrupted_mul_instructions(TEST_DATA));
    }

    const TEST_DATA_2: &str = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn sum_mul_should_return_48_for_test_data() {
        assert_eq!(48, sum_mul(TEST_DATA_2));
    }
}
