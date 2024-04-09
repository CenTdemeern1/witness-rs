#[derive(Debug, Clone, Copy)]
pub enum Vertex {
    /// This vertex does not exist on the grid.
    None,
    /// This is a standard vertex with nothing special going on.
    Vertex,
    /// This is a vertex with a dot on it, meaning the solution requires the line drawing over it.
    Dot,
}
