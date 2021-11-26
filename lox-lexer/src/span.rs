/// The line in the source stream
pub type Line = usize;

/// The column in the source stream
pub type Column = usize;

/// The position in the stream
pub struct Position {
    ln: Line,
    col: Column,
}
