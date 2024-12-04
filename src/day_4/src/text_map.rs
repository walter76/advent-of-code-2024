#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextMap {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl TextMap {
    pub fn char_at(&self, x: usize, y: usize) -> char {
        self.data[self.index_of(x, y)]
    }

    fn index_of(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl From<&str> for TextMap {
    fn from(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines.iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);
        let data: Vec<char> = lines.iter()
            .flat_map(|line| line.chars()).collect();

        Self {
            data,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::text_map::TextMap;

    const EXAMPLE_MAP: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn text_map_from_str_should_calculate_10_as_width() {
        let text_map: TextMap = TextMap::from(EXAMPLE_MAP);

        assert_eq!(10, text_map.width);
    }

    #[test]
    fn text_map_from_str_should_calculate_10_as_height() {
        let text_map: TextMap = TextMap::from(EXAMPLE_MAP);

        assert_eq!(10, text_map.height);
    }

    #[test]
    fn text_map_from_str_should_have_data_as_flat_array() {
        let text_map: TextMap = TextMap::from(EXAMPLE_MAP);

        let expected: &str = r"MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";

        assert_eq!(expected.chars().collect::<Vec<char>>(), text_map.data);
    }

    #[test]
    fn char_at_should_return_m_for_0_0() {
        let text_map: TextMap = TextMap::from(EXAMPLE_MAP);

        assert_eq!('M', text_map.char_at(0, 0));
    }

    #[test]
    fn char_at_should_return_m_for_9_0() {
        let text_map: TextMap = TextMap::from(EXAMPLE_MAP);

        assert_eq!('M', text_map.char_at(9, 0));
    }

    #[test]
    fn char_at_should_return_m_for_0_9() {
        let text_map: TextMap = TextMap::from(EXAMPLE_MAP);

        assert_eq!('M', text_map.char_at(0, 9));
    }

    #[test]
    fn char_at_should_return_x_for_9_9() {
        let text_map: TextMap = TextMap::from(EXAMPLE_MAP);

        assert_eq!('X', text_map.char_at(9, 9));
    }
}
