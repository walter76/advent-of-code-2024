use crate::primitives::Position;

/// A 2D grid of integers.
#[derive(Debug, Clone)]
pub struct IntGrid {
    /// The data of the grid.
    data: Vec<i64>,

    /// The width of the grid.
    width: usize,

    /// The height of the grid.
    height: usize,
}

impl IntGrid {
    /// Creates a new `IntGrid` with the given `width`, `height`, and `init_i`.
    /// All cells are initialized with `init_i`.
    /// 
    /// # Arguments
    /// 
    /// - `width` - The width of the grid.
    /// - `height` - The height of the grid.
    /// - `init_i` - The integer to initialize all cells with.
    pub fn new(width: usize, height: usize, init_i: i64) -> Self {
        Self {
            data: vec![init_i; width * height],
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
    
    /// Returns the integer at the given `pos`.
    /// 
    /// # Arguments
    /// 
    /// - `pos` - The position to get the integer from.
    pub fn get(&self, pos: Position) -> i64 {
        self.data[pos.y * self.width + pos.x]
    }

    /// Sets the integer at the given `pos` to `i`.
    /// 
    /// # Arguments
    /// 
    /// - `pos` - The position to set the integer at.
    /// - `i` - The integer to set.
    pub fn set(&mut self, pos: Position, i: i64) {
        self.data[pos.y * self.width + pos.x] = i;
    }

    /// Returns the sum of all integers in the grid.
    pub fn sum(&self) -> i64 {
        self.data.iter().sum()
    }

    /// Returns an iterator over the positions and integers in the grid.
    /// The iterator returns the position and integer as a tuple.
    pub fn iter(&self) -> impl Iterator<Item = (Position, i64)> + '_ {
        self.data.iter().enumerate().map(move |(i, &value)| {
            let x = i % self.width;
            let y = i / self.width;
            (Position { x, y }, value)
        })
    }
}

#[cfg(test)]
mod test {
    use super::IntGrid;
    use crate::primitives::Position;

    #[test]
    fn new_should_create_grid_with_correct_size() {
        let grid = IntGrid::new(3, 2, 0);
        
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.data.len(), 6);
    }

    #[test]
    fn new_should_initialize_grid_with_correct_integer() {
        let grid = IntGrid::new(3, 2, 42);
        
        assert_eq!(grid.data, vec![42; 6]);
    }

    #[test]
    fn get_should_return_correct_integer() {
        let mut grid = IntGrid::new(3, 2, 0);

        grid.set(Position { x: 1, y: 0 }, 42);
        
        assert_eq!(grid.get(Position { x: 1, y: 0 }), 42);
    }

    #[test]
    fn set_should_set_correct_integer() {
        let mut grid = IntGrid::new(3, 2, 0);

        grid.set(Position { x: 1, y: 0 }, 42);
        
        assert_eq!(grid.data, vec![0, 42, 0, 0, 0, 0]);
    }

    #[test]
    fn sum_should_return_correct_sum() {
        let grid = IntGrid {
            data: vec![1, 2, 3, 4, 5, 6],
            width: 3,
            height: 2,
        };

        assert_eq!(grid.sum(), 21);
    }
}
