fn main() {
    println!("Hello, world!");
}

fn parse_location_ids(puzzle_input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut location_ids_1 = vec![];
    let mut location_ids_2 = vec![];

    for line in puzzle_input.lines() {
        let (id_1, id_2) = parse_line(line);

        location_ids_1.push(id_1);
        location_ids_2.push(id_2);
    }

    (location_ids_1, location_ids_2)
}

fn parse_line(line: &str) -> (i32, i32) {
    let parts: Vec<String> =
        line.split_whitespace().map(|s| s.to_string()).collect();

    let x: i32 = parts[0].parse().unwrap();
    let y: i32 = parts[1].parse().unwrap();

    (x, y)
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, parse_location_ids};

    #[test]
    fn parse_line_should_return_pair_of_3_and_4() {
        assert_eq!((3, 4), parse_line("3   4"));
    }

    #[test]
    fn parse_location_ids_should_return_vecs() {
        assert_eq!((vec![3], vec![4]), parse_location_ids("3   4"));
    }

    const TEST_DATA: &str =
r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn parse_location_ids_should_parse_test_data() {
        assert_eq!(
            (
                vec![3, 4, 2, 1, 3, 3],
                vec![4, 3, 5, 3, 9, 3],
            ),
            parse_location_ids(TEST_DATA)
        );
    }
}