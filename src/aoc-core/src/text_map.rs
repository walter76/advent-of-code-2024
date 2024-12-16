use crate::primitives::Rect;

/// A 2D grid of characters.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextMap {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl TextMap {
    /// Returns the character at the given `x` and `y` coordinates.
    /// 
    /// # Arguments
    /// 
    /// - `x` - The x-coordinate.
    /// - `y` - The y-coordinate.
    pub fn char_at(&self, x: usize, y: usize) -> char {
        self.data[self.index_of(x, y)]
    }

    /// Returns the index of the given `x` and `y` coordinates.
    /// 
    /// # Arguments
    /// 
    /// - `x` - The x-coordinate.
    /// - `y` - The y-coordinate.
    fn index_of(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Sets the character at the given `x` and `y` coordinates to `c`. This
    /// method is not implementing any bounds checking.
    /// 
    /// # Arguments
    /// 
    /// - `x` - The x-coordinate.
    /// - `y` - The y-coordinate.
    /// - `c` - The character to set.
    pub fn set_char(&mut self, x: usize, y: usize, c: char) {
        let index = self.index_of(x, y);

        self.data[index] = c;
    }

    /// Returns a slice of the grid from `(x1, y1)` to `(x2, y2)`. The method
    /// can only handle slices that are horizontal, vertical, or diagonal with
    /// a slope of 1 or -1. Other diagonal slices are not possible.
    /// 
    /// The method is checking the bounds of the slice and panics if the slice
    /// is out of bounds.
    /// 
    /// # Arguments
    /// 
    /// - `x1` - The x-coordinate of the first point.
    /// - `y1` - The y-coordinate of the first point.
    /// - `x2` - The x-coordinate of the second point.
    /// - `y2` - The y-coordinate of the second point.
    pub fn slice(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> String {
        let mut result = String::new();

        // bounds check
        if x1 > self.width - 1
            || x2 > self.width - 1
            || y1 > self.height - 1
            || y2 > self.height - 1
        {
            panic!(
                "Access is out of bounds: (x1={}, y1={})(x2={}, y2={}) (width={}, height={})!",
                x1, y1, x2, y2, self.width, self.height);
        }

        if y1 == y2 {
            if x2 > x1 {
                // EAST

                for xn in x1..=x2 {
                    result.push(self.char_at(xn, y1));
                }
            } else if x2 < x1 {
                // WEST

                for xn in (x2..=x1).rev() {
                    result.push(self.char_at(xn, y1));
                }
            } else {
                // CENTER y1 == y2 && x1 == x2

                result.push(self.char_at(x1, y1));
            }
        } else if y2 < y1 {
            if x1 == x2 {
                // NORTH

                for yn in (y2..=y1).rev() {
                    result.push(self.char_at(x2, yn));
                }
            } else if x2 > x1 {
                // NORTH-EAST

                let mut xn = x1;

                for yn in (y2..=y1).rev() {
                    result.push(self.char_at(xn, yn));

                    xn += 1;
                }
            } else {
                // NORTH-WEST

                let mut xn = x1;

                for yn in (y2..=y1).rev() {
                    result.push(self.char_at(xn, yn));

                    if xn > 0 {
                        xn -= 1;
                    }
                }
            }
        } else {
            if x1 == x2 {
                // SOUTH

                for yn in y1..=y2 {
                    result.push(self.char_at(x1, yn));
                }
            } else if x2 > x1 {
                // SOUTH-EAST

                let mut xn = x1;

                for yn in y1..=y2 {
                    result.push(self.char_at(xn, yn));

                    xn += 1;
                }
            } else {
                // SOUTH-WEST

                let mut xn = x1;

                for yn in y1..=y2 {
                    result.push(self.char_at(xn, yn));

                    if xn > 0 {
                        xn -= 1;
                    }
                }
            }
        }

        result
    }

    /// Returns a new `TextMap` that represents the rectangle defined by `r`.
    /// Only rectangles are allowed that are normalized, i. e. the upper left
    /// corner is `(x1, y1)` and the lower right corner is `(x2, y2)`. The
    /// method panics if the rectangle is not normalized.
    /// 
    /// The method is checking the bounds of the rectangle and panics if the
    /// rectangle is out of bounds.
    /// 
    /// # Arguments
    /// 
    /// - `r` - The rectangle to get.
    pub fn rect(&self, r: Rect) -> TextMap {
        if r.x2 < r.x1 || r.y2 < r.y1 {
            panic!("Only rectangles allowed, where the rectangle is normalized (x2 < x1 || y2 < y1): {:?}", r);
        }

        if r.x2 > self.width - 1 || r.y2 > self.height - 1 {
            panic!("Rectangle is out of bounds: r={:?} (width={}, height={})", r, self.width, self.height);
        }

        let mut result = String::new();

        for y in r.y1..=r.y2 {
            for x in r.x1..=r.x2 {
                result.push(self.char_at(x, y));
            }

            result.push('\n');
        }

        TextMap::from(result.as_str())
    }

    /// Returns the position of the given character `c` in the grid. The
    /// position is a tuple of the x and y coordinates. If the character is
    /// not found, `None` is returned.
    /// 
    /// # Arguments
    /// 
    /// - `c` - The character to find.
    pub fn find_char_pos(&self, c: char) -> Option<(usize, usize)> {
        if let Some(index) = self.data.iter().position(|ch| ch == &c) {
            let y = index / self.width;
            let x = index % self.width;

            Some((x, y))
        } else {
            None
        }
    }

    /// Returns the number of occurences of the given character `c` in the grid.
    /// 
    /// # Arguments
    /// 
    /// - `c` - The character to count.
    pub fn count_chars(&self, c: char) -> usize {
        self.data.iter().filter(|&ch| ch == &c).count()
    }    
}

impl From<&str> for TextMap {
    /// Creates a new `TextMap` from the given string. The string is expected
    /// to have line breaks at the end of each line.
    /// 
    /// # Arguments
    /// 
    /// - `s` - The string to create the `TextMap` from.
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
    use crate::{primitives::Rect, text_map::TextMap};

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

const TEST_DATA_CENTRAL: &str = r"..........
....X.....
..........
..........";
    
    #[test]
    fn slice_should_get_central_slice() {
        let text_map = TextMap::from(TEST_DATA_CENTRAL);

        assert_eq!("X", text_map.slice(4, 1, 4, 1));
    }

const TEST_DATA_NORTH: &str = r"..S.......
..A.......
..M.......
..X.......";
    
    #[test]
    fn slice_should_get_northern_slice() {
        let text_map = TextMap::from(TEST_DATA_NORTH);

        assert_eq!("XMAS", text_map.slice(2, 3, 2, 0));
    }

    const TEST_DATA_NORTH_EAST: &str = r".....S....
....A.....
...M......
..X.......";
    
    #[test]
    fn slice_should_get_north_eastern_slice() {
        let text_map = TextMap::from(TEST_DATA_NORTH_EAST);

        assert_eq!("XMAS", text_map.slice(2, 3, 5, 0));
    }

    const TEST_DATA_EAST: &str = r"......XMAS
..........
..........
..........";
    
    #[test]
    fn slice_should_get_eastern_slice() {
        let text_map = TextMap::from(TEST_DATA_EAST);

        assert_eq!("XMAS", text_map.slice(6, 0, 9, 0));
    }

    const TEST_DATA_SOUTH_EAST: &str = r"X.........
.M........
..A.......
...S......";
    
    #[test]
    fn slice_should_get_south_eastern_slice() {
        let text_map = TextMap::from(TEST_DATA_SOUTH_EAST);

        assert_eq!("XMAS", text_map.slice(0, 0, 3, 3));
    }

    const TEST_DATA_SOUTH: &str = r".........X
.........M
.........A
.........S";

    #[test]
    fn slice_should_get_southern_slice() {
        let text_map = TextMap::from(TEST_DATA_SOUTH);

        assert_eq!("XMAS", text_map.slice(9, 0, 9, 3));
    }

    const TEST_DATA_SOUTH_WEST: &str = r"...X......
..M.......
.A........
S.........";

    #[test]
    fn slice_should_get_south_western_slice() {
        let text_map = TextMap::from(TEST_DATA_SOUTH_WEST);

        assert_eq!("XMAS", text_map.slice(3, 0, 0, 3));
    }

    const TEST_DATA_WEST: &str = r"..........
..........
..........
SAMX......";

    #[test]
    fn slice_should_get_western_slice() {
        let text_map = TextMap::from(TEST_DATA_WEST);

        assert_eq!("XMAS", text_map.slice(3, 3, 0, 3));
    }

    const TEST_DATA_NORTH_WEST: &str = r"S.........
.A........
..M.......
...X......";
    
    #[test]
    fn slice_should_get_north_western_slice() {
        let text_map = TextMap::from(TEST_DATA_NORTH_WEST);

        assert_eq!("XMAS", text_map.slice(3, 3, 0, 0));
    }

    #[test]
    fn slice_should_panics_when_x1_out_of_bounds() {
        let text_map = TextMap::from(TEST_DATA_NORTH_WEST);

        assert!(std::panic::catch_unwind(|| text_map.slice(10, 0, 0, 3)).is_err());
    }

    #[test]
    fn slice_should_panics_when_y1_out_of_bounds() {
        let text_map = TextMap::from(TEST_DATA_NORTH_WEST);

        assert!(std::panic::catch_unwind(|| text_map.slice(0, 10, 0, 3)).is_err());
    }

    #[test]
    fn slice_should_panics_when_x2_out_of_bounds() {
        let text_map = TextMap::from(TEST_DATA_NORTH_WEST);

        assert!(std::panic::catch_unwind(|| text_map.slice(0, 0, 10, 3)).is_err());
    }

    #[test]
    fn slice_should_panics_when_y2_out_of_bounds() {
        let text_map = TextMap::from(TEST_DATA_NORTH_WEST);

        assert!(std::panic::catch_unwind(|| text_map.slice(0, 0, 0, 10)).is_err());
    }

    const TEST_DATA_RECT: &str = r"..........
..........
..........
..........";

    #[test]
    fn rect_should_panic_when_x2_less_than_x1() {
        let text_map = TextMap::from(TEST_DATA_RECT);

        assert!(std::panic::catch_unwind(|| text_map.rect(Rect::new(1, 1, 0, 0))).is_err());
    }

    #[test]
    fn rect_should_panic_when_y2_less_than_y1() {
        let text_map = TextMap::from(TEST_DATA_RECT);

        assert!(std::panic::catch_unwind(|| text_map.rect(Rect::new(1, 1, 0, 0))).is_err());
    }

    #[test]
    fn rect_should_panic_when_x2_out_of_bounds() {
        let text_map = TextMap::from(TEST_DATA_RECT);

        assert!(std::panic::catch_unwind(|| text_map.rect(Rect::new(0, 0, 10, 0))).is_err());
    }

    #[test]
    fn rect_should_panic_when_y2_out_of_bounds() {
        let text_map = TextMap::from(TEST_DATA_RECT);

        assert!(std::panic::catch_unwind(|| text_map.rect(Rect::new(0, 0, 0, 10))).is_err());
    }

    const TEST_DATA_RECT_1: &str = r"..234.....
..567.....
..890.....
..........";

    #[test]
    fn rect_should_get_rect_1() {
        let text_map = TextMap::from(TEST_DATA_RECT_1);

        assert_eq!(
            TextMap::from("234\n567\n890"),
            text_map.rect(Rect::new(2, 0, 4, 2))
        );
    }

    const TEST_DATA_FIND_POS: &str = r"..........
....X.....
..........
..........";

    #[test]
    fn find_char_pos_should_return_none_for_not_found() {
        let text_map = TextMap::from(TEST_DATA_FIND_POS);

        assert_eq!(None, text_map.find_char_pos('A'));
    }

    #[test]
    fn find_char_pos_should_return_4_1_for_x() {
        let text_map = TextMap::from(TEST_DATA_FIND_POS);

        assert_eq!(Some((4, 1)), text_map.find_char_pos('X'));
    }

    const TEST_DATA_COUNT_OCCURENCES: &str = r"XMASXMASX
XMASXMASX
XMASXMASX
XMASXMASX";

    #[test]
    fn count_chars_should_return_12_for_x() {
        let text_map = TextMap::from(TEST_DATA_COUNT_OCCURENCES);

        assert_eq!(12, text_map.count_chars('X'));
    }

    #[test]
    fn count_chars_should_return_8_for_m() {
        let text_map = TextMap::from(TEST_DATA_COUNT_OCCURENCES);

        assert_eq!(8, text_map.count_chars('M'));
    }

    #[test]
    fn count_chars_should_return_8_for_a() {
        let text_map = TextMap::from(TEST_DATA_COUNT_OCCURENCES);

        assert_eq!(8, text_map.count_chars('A'));
    }

    #[test]
    fn count_chars_should_return_8_for_s() {
        let text_map = TextMap::from(TEST_DATA_COUNT_OCCURENCES);

        assert_eq!(8, text_map.count_chars('S'));
    }

    #[test]
    fn count_chars_should_return_0_for_not_found() {
        let text_map = TextMap::from(TEST_DATA_COUNT_OCCURENCES);

        assert_eq!(0, text_map.count_chars('Z'));
    }
}
