fn parse_page_updates(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "75,47,61,53,29";

    #[test]
    fn parse_page_updates_should_parse_test_data() {
        assert_eq!(vec![75, 47, 61, 53, 29], parse_page_updates(TEST_DATA));
    }
}
