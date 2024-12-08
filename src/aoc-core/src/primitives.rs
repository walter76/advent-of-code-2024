/// The position of something in a 2D grid.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    /// The x-coordinate of the position.
    pub x: usize,
    
    /// The y-coordinate of the position.
    pub y: usize,
}

/// A rectangle.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rect {
    /// The x-coordinate of the upper left corner.
    pub x1: usize,

    /// The y-coordinate of the upper left corner.
    pub y1: usize,

    /// The x-coordinate of the lower right corner.
    pub x2: usize,

    /// The y-coordinate of the lower right corner.
    pub y2: usize,
}

impl Rect {
    /// Creates a new `Rect` with the given coordinates.
    /// 
    /// # Arguments
    /// 
    /// - `x1` - The x-coordinate of the upper left corner.
    /// - `y1` - The y-coordinate of the upper left corner.
    /// - `x2` - The x-coordinate of the lower right corner.
    /// - `y2` - The y-coordinate of the lower right corner.
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
        }
    }
}
