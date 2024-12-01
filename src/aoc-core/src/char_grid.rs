use crate::primitives::Position;

/// A 2D grid of characters.
#[derive(Debug, Clone)]
pub struct CharGrid {
    /// The data of the grid.
    data: Vec<char>,

    /// The width of the grid.
    width: usize,

    /// The height of the grid.
    height: usize,
}

impl CharGrid {
    /// Creates a new `CharGrid` with the given `width`, `height`, and `init_char`.
    /// All cells are initialized with `init_char`.
    /// 
    /// # Arguments
    /// 
    /// - `width` - The width of the grid.
    /// - `height` - The height of the grid.
    /// - `init_char` - The character to initialize all cells with.
    pub fn new(width: usize, height: usize, init_char: char) -> Self {
        Self {
            data: vec![init_char; width * height],
            width,
            height,
        }
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }
    
    /// Returns the character at the given `pos`.
    /// 
    /// # Arguments
    /// 
    /// - `pos` - The position to get the character from.
    pub fn get(&self, pos: Position) -> char {
        self.data[pos.y * self.width + pos.x]
    }

    /// Sets the character at the given `pos` to `c`.
    /// 
    /// # Arguments
    /// 
    /// - `pos` - The position to set the character at.
    /// - `c` - The character to set.
    pub fn set(&mut self, pos: Position, c: char) {
        self.data[pos.y * self.width + pos.x] = c;
    }

    /// Fills the rectangle defined by `top_left` and `bottom_right` with `c`.
    /// 
    /// # Arguments
    /// 
    /// - `top_left` - The top-left corner of the rectangle.
    /// - `bottom_right` - The bottom-right corner of the rectangle.
    /// - `c` - The character to fill the rectangle with.
    pub fn fill_rect(&mut self, top_left: Position, bottom_right: Position, c: char) {
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                self.set(Position { x, y }, c);
            }
        }
    }

    /// Counts the occurrences of `c` in the grid.
    /// 
    /// # Arguments
    /// 
    /// - `c` - The character to count the occurrences of.
    pub fn count_occurrences(&self, c: char) -> usize {
        self.data.iter().filter(|&&x| x == c).count()
    }
}

#[cfg(test)]
mod test {
    use super::CharGrid;
    use crate::primitives::Position;

    #[test]
    fn new_should_create_grid_with_correct_size() {
        let grid = CharGrid::new(3, 2, 'a');
        
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.data.len(), 6);
    }

    #[test]
    fn new_should_initialize_grid_with_correct_char() {
        let grid = CharGrid::new(3, 2, 'a');
        
        assert_eq!(grid.data, vec!['a'; 6]);
    }

    #[test]
    fn get_should_return_correct_char() {
        let mut grid = CharGrid::new(3, 2, 'a');

        grid.set(Position { x: 1, y: 0 }, 'b');
        
        assert_eq!(grid.get(Position { x: 1, y: 0 }), 'b');
    }

    #[test]
    fn set_should_set_correct_char() {
        let mut grid = CharGrid::new(3, 2, 'a');

        grid.set(Position { x: 1, y: 0 }, 'b');
        
        assert_eq!(grid.data, vec!['a', 'b', 'a', 'a', 'a', 'a']);
    }

    #[test]
    fn fill_rect_should_fill_correct_rect() {
        let mut grid = CharGrid::new(3, 2, 'a');

        grid.fill_rect(Position { x: 0, y: 0 }, Position { x: 2, y: 1 }, 'b');
        
        assert_eq!(grid.data, vec!['b', 'b', 'b', 'b', 'b', 'b']);
    }

    #[test]
    fn count_occurrences_should_return_correct_count() {
        let grid = CharGrid {
            data: vec!['a', 'b', 'a', 'b', 'a', 'b'],
            width: 3,
            height: 2,
        };

        assert_eq!(grid.count_occurrences('a'), 3);
        assert_eq!(grid.count_occurrences('b'), 3);
    }
}
