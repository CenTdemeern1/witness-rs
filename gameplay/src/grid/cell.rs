use std::{rc::Rc, sync::RwLock};

use super::{color::Color, EdgeRef, VertexID};

pub type CellRef = Rc<RwLock<Cell>>;

#[derive(Debug, Clone)]
pub struct Cell {
    edges: Vec<EdgeRef>,
    pub kind: CellType,
}

impl Cell {
    pub fn new(edges: Vec<EdgeRef>) -> Self {
        Cell {
            edges,
            kind: CellType::Blank,
        }
    }

    pub fn new_of_kind(edges: Vec<EdgeRef>, kind: CellType) -> Self {
        Cell { edges, kind }
    }

    /// Checks whether the given edge borders this cell.
    pub fn has_edge(&self, edge: &EdgeRef) -> bool {
        self.edges
            .iter()
            .any(|e: &EdgeRef| EdgeRef::ptr_eq(e, edge))
    }

    /// Checks whether the given vertex borders this cell.
    /// Assumes this cell's edges can currently be read.
    pub fn has_vertex(&self, vertex: VertexID) -> bool {
        self.edges
            .iter()
            .any(|e: &EdgeRef| e.read().unwrap().connects_to(vertex))
    }

    /// Gets an immutable reference to the vector of edges that border this cell.
    pub fn get_edges(&self) -> &Vec<EdgeRef> {
        &self.edges
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    /// This is a blank cell.
    Blank,
    /// This is a cell with a colored square in it, which needs to be segregated from symbols of other colors.
    Square(Color),
    /// This is a cell with a star in it, which needs to be paired up with exactly one other symbol of the same color.
    /// It does not need to be segregated from symbols of other colors.
    Star(Color),
    /// Triangle
    Triangle(TriangleCount),
}

/// This enum only has three possible values, for the three possible triangle sets that can be in a cell.
/// This helps with ensuring type safety of triangles, by limiting the possible values.
/// This can be cast into a number value by using the `as` keyword.
#[derive(Debug, Clone, Copy)]
pub enum TriangleCount {
    One = 1,
    Two = 2,
    Three = 3,
}

impl TryFrom<u8> for TriangleCount {
    /// This only has one way to fail and it's pretty obvious; passing in an invalid value.
    /// This also means we're not wasting any memory on a fail state enum with only one possible value, which would be pretty useless.
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TriangleCount::One),
            2 => Ok(TriangleCount::Two),
            3 => Ok(TriangleCount::Three),
            _ => Err(()),
        }
    }
}
