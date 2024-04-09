#[derive(Debug, Clone, Copy)]
pub enum Edge {
    /// This edge does not exist on the grid.
    None,
    /// This is an edge that's mostly present but has a gap in the middle so you can't fully cross it; a gap in the road.
    Gap,
    /// This is a standard edge with nothing special going on.
    Edge,
    /// This is a edge with a dot on it, meaning the solution requires the line drawing over it.
    Dot,
}
