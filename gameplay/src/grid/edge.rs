use std::{rc::Rc, sync::RwLock};

use super::VertexID;

pub type EdgeRef = Rc<RwLock<Edge>>;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    from: VertexID,
    to: VertexID,
    kind: EdgeType
}

impl Edge {
    pub fn new(from: VertexID, to: VertexID) -> Self {
        Edge {
            from: from.min(to),
            to: to.max(from),
            kind: EdgeType::Edge,
        }
    }

    pub fn new_of_kind(from: VertexID, to: VertexID, kind: EdgeType) -> Self {
        Edge {
            from: from.min(to),
            to: to.max(from),
            kind,
        }
    }

    /// Returns whether this edge is connected to the given vertex.
    pub fn connects_to(&self, id: VertexID) -> bool {
        self.from == id || self.to == id
    }

    /// Gets the vertex IDs of the vertices this edge is connected to.
    /// IDs are sorted from low to high numerically.
    #[deprecated = "This will likely not be used and it assumes that IDs are numerical. Considered for removal."]
    pub fn get_vertices(&self) -> [VertexID; 2] {
        [self.from, self.to]
    }
}

/// Describes the kind of edge it's attached to.
#[derive(Debug, Clone, Copy)]
pub enum EdgeType {
    /// This is an edge that's mostly present but has a gap in the middle so you can't fully cross it; a gap in the road.
    Gap,
    /// This is a standard edge with nothing special going on.
    Edge,
    /// This is a edge with a dot on it, meaning the solution requires the line drawing over it.
    Dot,
}
