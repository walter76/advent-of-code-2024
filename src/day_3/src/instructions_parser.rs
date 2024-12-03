use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Instruction {
    Do,
    Dont,
    Mul(i32, i32),
}

pub fn parse(puzzle_input: &str) -> Option<Vec<Instruction>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut instructions: Vec<Instruction> = vec![];

    let mut start = 0;

    while start < puzzle_input.len() {
        if (start + 3) < puzzle_input.len()
            && &puzzle_input[start .. start + 4] == "mul("
        {
            if let Some(end_pos) = puzzle_input[start ..].find(')') {
                // println!("[{}]: start: {}, end: {}", &puzzle_input[start ..], start, start + end_pos);

                if let Some(cap) = re.captures(&puzzle_input[start .. start + end_pos + 1]) {
                    let num1: i32 = cap[1].parse().expect("Failed to parse num1");
                    let num2: i32 = cap[2].parse().expect("Failed to parse num2");
        
                    instructions.push(Instruction::Mul(num1, num2));

                    // found a valid mul instruction, moving past the instruction
                    start = start + end_pos + 1;
                } else {
                    // println!("no captures in '{}'", &puzzle_input[start .. end_pos]);

                    // found a invalid mul instruction or something else, moving past the `mul(`
                    start += 4;
                }
            } else {
                // println!("no closing bracket found");

                // found no closing bracket, moving past the `mul(`
                start += 4;
            }
        } else if (start + 3) < puzzle_input.len()
            && &puzzle_input[start .. start + 4] == "do()"
        {
            instructions.push(Instruction::Do);

            start += 4;
        } else if (start + 6) < puzzle_input.len()
            && &puzzle_input[start .. start + 7] == "don't()"
        {
            instructions.push(Instruction::Dont);

            start += 7;
        } else {
            // println!("no instruction found");

            // no instruction found, increasing index by one
            start += 1;
        }
    }

    if instructions.is_empty() {
        None
    } else {
        Some(instructions)
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions_parser::Instruction;

    use super::parse;

    #[test]
    fn parse_should_parse_regular_mul_instructions() {
        assert_eq!(Some(vec![Instruction::Mul(44,46)]), parse("mul(44,46)"));
        assert_eq!(Some(vec![Instruction::Mul(123,4)]), parse("mul(123,4)"));
    }

    #[test]
    fn parse_should_ignore_wrong_mul_instructions() {
        assert_eq!(None, parse("mul(4*"));
        assert_eq!(None, parse("mul(6,9!"));
        assert_eq!(None, parse("?(12,34)"));
        assert_eq!(None, parse("mul ( 2 , 4 )"));
    }

    #[test]
    fn parse_should_parse_do_instruction() {
        assert_eq!(Some(vec![Instruction::Do]), parse("do()"));
    }

    #[test]
    fn parse_should_parse_dont_instruction() {
        assert_eq!(Some(vec![Instruction::Dont]), parse("don't()"));
    }

    const TEST_DATA: &str = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    
    #[test]
    fn parse_should_parse_test_data() {
        assert_eq!(
            Some(
                vec![
                    Instruction::Mul(2,4),
                    Instruction::Dont,
                    Instruction::Mul(5,5),
                    Instruction::Mul(11,8),
                    Instruction::Do,
                    Instruction::Mul(8,5),
                ]
            ),
            parse(TEST_DATA)
        )
    }
}