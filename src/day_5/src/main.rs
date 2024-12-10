mod parser;
mod rule;

use anyhow::Result;
use rule::PageOrderingRule;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

fn parse_input(puzzle_input: &str) -> (Vec<PageOrderingRule>, Vec<Vec<i32>>) {
    let mut page_ordering_rules: Vec<PageOrderingRule> = vec![];
    let mut page_updates: Vec<Vec<i32>> = vec![];

    let mut line_iter = puzzle_input.lines().peekable();

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }

        page_ordering_rules.push(PageOrderingRule::from(line));
    }

    while let Some(line) = line_iter.next() {
        page_updates.push(parser::parse_page_updates(line));
    }

    (page_ordering_rules, page_updates)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, rule::PageOrderingRule};

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
    fn parse_input_should_parse_page_ordering_rules_in_example_data() {
        let (page_ordering_rules, _) = parse_input(EXAMPLE_DATA);

        assert_eq!(
            vec![
                PageOrderingRule::new(47, 53),
                PageOrderingRule::new(97, 13),
                PageOrderingRule::new(97, 61),
                PageOrderingRule::new(97, 47),
                PageOrderingRule::new(75, 29),
                PageOrderingRule::new(61, 13),
                PageOrderingRule::new(75, 53),
                PageOrderingRule::new(29, 13),
                PageOrderingRule::new(97, 29),
                PageOrderingRule::new(53, 29),
                PageOrderingRule::new(61, 53),
                PageOrderingRule::new(97, 53),
                PageOrderingRule::new(61, 29),
                PageOrderingRule::new(47, 13),
                PageOrderingRule::new(75, 47),
                PageOrderingRule::new(97, 75),
                PageOrderingRule::new(47, 61),
                PageOrderingRule::new(75, 61),
                PageOrderingRule::new(47, 29),
                PageOrderingRule::new(75, 13),
                PageOrderingRule::new(53, 13),
            ],
            page_ordering_rules
        );
    }

    #[test]
    fn parse_input_should_parse_page_updates_in_example_data() {
        let (_, page_updates) = parse_input(EXAMPLE_DATA);

        assert_eq!(
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
            page_updates
        )
    }
}