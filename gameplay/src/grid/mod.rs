pub mod vertex;
pub mod edge;
pub mod cell;
pub mod color;
use witness_core::{Abs, Vector2};

use vertex::Vertex;
use edge::Edge;
use cell::Cell;

/// The unit of measurement for grid coordinates.
/// This is currently a Vector2<u8> because I assume puzzles won't need to be bigger than 255x255
pub type GridVector2 = Vector2<usize>;

pub trait Vector2Extensions {
    fn is_immediately_next_to(&self, other: &Self) -> bool;
}

impl Vector2Extensions for GridVector2 {
    /// Checks if this grid position is immediately next to another,
    /// meaning directly one unit above, below, to the left of, or to the right of the other position
    /// (but not diagonally)
    fn is_immediately_next_to(&self, other: &GridVector2) -> bool {
        // These casts ensure no information loss when working with negative number math
        (vec2cast!(self, isize) - vec2cast!(other, isize)).abs().sum() == 1
    }
}

pub struct Grid {
    size: GridVector2,
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    cells: Vec<Cell>,
}

impl Grid {
    /// Checks whether a vertex is in-bounds.
    pub fn is_vertex_in_bounds(&self, position: GridVector2) -> bool {
        // Assumes GridVector2 can't go negative.
        vertex.max(self.size) == self.size
    }

    /// Returns a copy of the vertex at the given position, if it exists.
    pub fn get_vertex(&self, position: GridVector2) -> Option<Vertex> {
        if self.is_vertex_in_bounds(position) {
            Some(self.vertices[position.x + (self.size.x + 1) * position.y])
        } else {
            None
        }
    }

    /// Returns a copy of the edge between two grid positions, if it exists.
    pub fn get_edge_between_vertices(&self, position1: GridVector2, position2: GridVector2) -> Option<Edge> {
        // Both vertices must be in-bounds and they must be directly adjacent.
        if self.is_vertex_in_bounds(position1) && self.is_vertex_in_bounds(position2) && position1.is_immediately_next_to(&position2) {
            let smallest = position1.min(position2);
            Some(
                self.edges[
                    smallest.x
                    + (self.size.x + self.size.y + 1) * smallest.y
                    + if position1.y == position2.y {
                        // The vertex closest to the bottom-right is to the right of the vertex closest to the top-left,
                        // meaning the edge is a horizontal edge.
                        0
                    } else {
                        // The vertex closest to the bottom-right is below the vertex closest to the top-left,
                        // meaning the edge is a vertical edge.
                        self.size.x
                    }
                ]
            )
        } else {
            None
        }
    }

    /// Checks whether a cell is in-bounds.
    pub fn is_cell_in_bounds(&self, position: GridVector2) -> bool {
        // Assumes GridVector2 can't go negative.
        position.min(self.size - vec2!(1)) == position
    }

    /// Returns a copy of the cell at the given grid position, if it exists.
    pub fn get_cell(&self, position: GridVector2) -> Option<Cell> {
        if self.is_cell_in_bounds(position) {
            Some(self.cells[position.x + self.size.x * position.y])
        } else {
            None
        }
    }
}
