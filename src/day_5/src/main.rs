mod parser;
mod rule;

use anyhow::Result;
use rule::PageOrderingRule;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

fn verify(page_updates: &[i32], page_ordering_rules: &[PageOrderingRule]) -> bool {
    for rule in page_ordering_rules.iter() {
        if !verify_update(page_updates, rule) {
            return false;
        }
    }

    true
}

fn verify_update(page_updates: &[i32], rule: &PageOrderingRule) -> bool {
    if let Some(n1_pos) = page_updates.iter().position(|p| p == &rule.n1) {
        if let Some(n2_pos) = page_updates.iter().position(|p| p == &rule.n2) {
            n1_pos < n2_pos
        } else {
            true
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse_input, verify};

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
    fn verify_should_return_true_for_first_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(verify(&all_page_updates[0], &page_ordering_rules));
    }

    #[test]
    fn verify_should_return_true_for_second_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(verify(&all_page_updates[1], &page_ordering_rules));
    }

    #[test]
    fn verify_should_return_true_for_third_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(verify(&all_page_updates[2], &page_ordering_rules));
    }

    #[test]
    fn verify_should_return_false_for_fourth_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(!verify(&all_page_updates[3], &page_ordering_rules));
    }

    #[test]
    fn verify_should_return_false_for_fifth_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(!verify(&all_page_updates[4], &page_ordering_rules));
    }

    #[test]
    fn verify_should_return_false_for_sixth_updates_in_example_data() {
        let (page_ordering_rules, all_page_updates) = parse_input(EXAMPLE_DATA);

        assert!(!verify(&all_page_updates[5], &page_ordering_rules));
    }
}
