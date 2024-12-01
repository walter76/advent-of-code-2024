/// The position of something in a 2D grid.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    /// The x-coordinate of the position.
    pub x: usize,
    
    /// The y-coordinate of the position.
    pub y: usize,
}
