mod parser;
mod rule;

use std::cmp::Ordering;

use anyhow::Result;
use parser::parse_input;
use rule::PageOrderingRule;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 5)?;

    let sum_of_middle_page_numbers =
        sum_middle_page_numbers_of_valid_page_updates(&puzzle_input);

    println!(
        "The sum of middle page numbers of valid page updates is: {}",
        sum_of_middle_page_numbers);

    Ok(())
}

fn sum_middle_page_numbers_of_valid_page_updates(puzzle_input: &str) -> i32 {
    let (page_ordering_rules, all_page_updates) = parse_input(puzzle_input);

    all_page_updates.iter()
        .filter(|page_update| verify_safety_manual_update(page_update, &page_ordering_rules))
        .map(|page_update| get_middle_page_number(page_update))
        .sum()
}

fn get_middle_page_number(page_updates: &[i32]) -> i32 {
    let index = page_updates.len() / 2;

    page_updates[index]
}

fn verify_safety_manual_update(safety_manual_update: &[i32], page_ordering_rules: &[PageOrderingRule]) -> bool {
    for page_ordering_rule in page_ordering_rules.iter() {
        if !verify_page_ordering_rule(safety_manual_update, page_ordering_rule) {
            return false;
        }
    }

    true
}

fn verify_page_ordering_rule(safety_manual_update: &[i32], page_ordering_rule: &PageOrderingRule) -> bool {
    if let Some(n1_pos) = safety_manual_update.iter().position(|page_number| page_number == &page_ordering_rule.n1) {
        if let Some(n2_pos) = safety_manual_update.iter().position(|page_number| page_number == &page_ordering_rule.n2) {
            n1_pos < n2_pos
        } else {
            true
        }
    } else {
        true
    }
}

fn sum_middle_page_numbers_of_invalid_page_updates(puzzle_input: &str) -> i32 {
    let (page_ordering_rules, all_page_updates) = parse_input(puzzle_input);

    all_page_updates.iter()
        .filter(|page_update| !verify_safety_manual_update(page_update, &page_ordering_rules))
        .map(|page_update| {
            let sorted = sort_by_page_order_rules(page_update, &page_ordering_rules);
            get_middle_page_number(&sorted)
        })
        .sum()
}

fn sort_by_page_order_rules(safety_manual_update: &Vec<i32>, page_ordering_rules: &[PageOrderingRule]) -> Vec<i32> {
    let compare = |x: &i32, y: &i32| {
        let (x, y) = (*x, *y);

        if page_ordering_rules.contains(&PageOrderingRule::new(x, y)) {
            Ordering::Less
        } else if page_ordering_rules.contains(&PageOrderingRule::new(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let mut sorted_safety_manual_update: Vec<i32> = safety_manual_update.clone();

    sorted_safety_manual_update.sort_by(compare);

    sorted_safety_manual_update
}

#[cfg(test)]
mod tests {
    use crate::{get_middle_page_number, parser::parse_input, sum_middle_page_numbers_of_invalid_page_updates, sum_middle_page_numbers_of_valid_page_updates, verify_safety_manual_update};

    const EXAMPLE_DATA: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn verify_safety_manual_update_should_return_true_for_first_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(verify_safety_manual_update(&all_page_updates[0], &page_ordering_rules));
    }

    #[test]
    fn verify_safety_manual_update_should_return_true_for_second_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(verify_safety_manual_update(&all_page_updates[1], &page_ordering_rules));
    }

    #[test]
    fn verify_safety_manual_update_should_return_true_for_third_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(verify_safety_manual_update(&all_page_updates[2], &page_ordering_rules));
    }

    #[test]
    fn verify_safety_manual_update_should_return_false_for_fourth_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(!verify_safety_manual_update(&all_page_updates[3], &page_ordering_rules));
    }

    #[test]
    fn verify_safety_manual_update_should_return_false_for_fifth_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(!verify_safety_manual_update(&all_page_updates[4], &page_ordering_rules));
    }

    #[test]
    fn verify_safety_manual_update_should_return_false_for_sixth_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(!verify_safety_manual_update(&all_page_updates[5], &page_ordering_rules));
    }

    #[test]
    fn get_middle_page_number_should_return_61_for_first_page_updates() {
        assert_eq!(61, get_middle_page_number(&vec![75,47,61,53,29]));
    }

    #[test]
    fn sum_middle_page_numbers_of_valid_page_updates_should_return_143_for_example_data() {
        assert_eq!(143, sum_middle_page_numbers_of_valid_page_updates(EXAMPLE_DATA));
    }

    #[test]
    fn sum_middle_page_numbers_of_invalid_page_updates_should_return_123_for_example_data() {
        assert_eq!(123, sum_middle_page_numbers_of_invalid_page_updates(EXAMPLE_DATA));
    }
}
