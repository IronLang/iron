/// Represents a specific location in a file.
///
/// Coordinates are primarily used to describe where an Iron `Token` starts and ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn new(row: usize, col: usize) -> Coordinate {
        Coordinate { row, col }
    }
}
