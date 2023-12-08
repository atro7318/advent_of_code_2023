#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SymbolCoordinate {
    row: usize,
    col: usize,
}

impl SymbolCoordinate {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
