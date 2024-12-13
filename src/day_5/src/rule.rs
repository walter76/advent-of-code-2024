#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PageOrderingRule {
    pub n1: i32,
    pub n2: i32,
}

impl PageOrderingRule {
    pub fn new(n1: i32, n2: i32) -> Self {
        Self { n1, n2 }
    }
}

impl From<&str> for PageOrderingRule {
    fn from(s: &str) -> Self {
        let tokens: Vec<&str> = s.split('|').collect();
    
        Self::new( 
            tokens[0].parse().unwrap(),
            tokens[1].parse().unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::PageOrderingRule;

    #[test]
    fn from_should_parse_47_53() {
        assert_eq!(
            PageOrderingRule { n1: 47, n2: 53 },
            PageOrderingRule::from("47|53"));
    }
}
