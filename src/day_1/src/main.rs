mod parser;

use anyhow::Result;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 1)?;

    let (location_ids_left, location_ids_right) =
        parser::parse_location_ids(&puzzle_input);

    let total_distance =
        total_distance(location_ids_left.clone(), location_ids_right.clone());

    println!("Total distance between the lists: {}", total_distance);

    let total_similarity =
        total_similarity(&location_ids_left, &location_ids_right);

    println!("Total similarity between the lists: {}", total_similarity);

    Ok(())
}

fn total_similarity(location_ids_left: &[i32], location_ids_right: &[i32])
    -> i32
{
    location_ids_left.iter()
        .map(|id| similarity(*id, location_ids_right)).sum()
}

fn similarity(location_id: i32, location_ids_right: &[i32]) -> i32 {
    let no_of_occurences =
        location_ids_right.iter().filter(|&id| *id == location_id).count();

    no_of_occurences as i32 *location_id
}

fn total_distance(
    mut location_ids_left: Vec<i32>, mut location_ids_right: Vec<i32>
) -> i32 {
    location_ids_left.sort();
    location_ids_right.sort();

    let pairs: Vec<(i32, i32)> =
        location_ids_left.into_iter().zip(
            location_ids_right.into_iter()
        ).collect();

    pairs.iter().map(|&pair| distance(pair)).sum()
}

fn distance(pair: (i32, i32)) -> i32 {
    if pair.0 > pair.1 {
        pair.0 - pair.1
    } else {
        pair.1 - pair.0
    }
}

const TEST_DATA: &str =
r"3   4
4   3
2   5
1   3
3   9
3   3";

#[cfg(test)]
mod tests {
    use crate::{distance, parser::parse_location_ids, similarity, total_distance, total_similarity, TEST_DATA};

    #[test]
    fn distance_should_return_2_for_pair_1_and_3() {
        assert_eq!(2, distance((1, 3)));
    }

    #[test]
    fn distance_should_return_2_for_pair_3_and_1() {
        assert_eq!(2, distance((3, 1)));
    }

    #[test]
    fn total_distance_of_test_data_should_be_11() {
        let (location_ids_left, location_ids_right) =
            parse_location_ids(TEST_DATA);

        assert_eq!(11, total_distance(location_ids_left, location_ids_right));
    }

    #[test]
    fn similarity_for_3_and_test_data_should_be_9() {
        let (_, location_ids_right) = parse_location_ids(TEST_DATA);

        assert_eq!(9, similarity(3, &location_ids_right));
    }

    #[test]
    fn similarity_for_2_and_test_data_should_be_0() {
        let (_, location_ids_right) = parse_location_ids(TEST_DATA);

        assert_eq!(0, similarity(2, &location_ids_right));
    }

    #[test]
    fn total_similarity_for_test_data_should_be_31() {
        let (location_ids_left, location_ids_right) =
            parse_location_ids(TEST_DATA);

        assert_eq!(
            31,
            total_similarity(&location_ids_left, &location_ids_right));
    }
}