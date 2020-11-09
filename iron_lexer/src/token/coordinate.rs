/// Represents a specific location in a file.
///
/// Coordinates are primarily used to describe where an Iron `Token` starts and ends.
/// It is possible that this will be deprecated in favor of a tuple type in the future,
/// but for now, this was the path of least resistance.
///
/// **Note that the `row` and `col` values are 0-indexed.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    /// Creates a new coordinate, referencing a given `row` and `col`.
    pub fn new(row: usize, col: usize) -> Coordinate {
        Coordinate { row, col }
    }
}
