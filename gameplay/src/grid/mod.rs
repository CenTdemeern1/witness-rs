pub mod cell;
pub mod color;
pub mod edge;
pub mod vertex;

use std::{collections::VecDeque, rc::Rc, sync::RwLock};

use witness_core::{Abs, Vector2};

pub use cell::{Cell, CellRef};
pub use color::Color;
pub use edge::{Edge, EdgeRef};
pub use vertex::{Vertex, VertexID};

/// The unit of measurement for grid coordinates.
/// This is currently a `Vector2<usize>`
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
        (vec2cast!(self, isize) - vec2cast!(other, isize))
            .abs()
            .sum()
            == 1
    }
}

/// A Witness puzzle.
pub struct Grid {
    size: GridVector2,
    vertices: Vec<Vertex>,
    edges: Vec<EdgeRef>,
    cells: Vec<CellRef>,
}

impl Grid {
    /// Creates a new puzzle with a given size.
    pub fn new(size: GridVector2) -> Self {
        let mut edges = vec![];
        (0..=size.x).for_each(|x| {
            (0..=size.y).for_each(|y| {
                let current_vec = vec2!(x, y);
                let current = Self::vector_to_vertex_id(size, current_vec);
                if current_vec.x != 0 {
                    edges.push(Edge::new(
                        current,
                        Self::vector_to_vertex_id(size, current_vec - vec2!(1, 0)),
                    ));
                }
                if current_vec.y != 0 {
                    edges.push(Edge::new(
                        current,
                        Self::vector_to_vertex_id(size, current_vec - vec2!(0, 1)),
                    ));
                }
            })
        });
        let edges: Vec<EdgeRef> = edges.into_iter().map(|x| Rc::new(RwLock::new(x))).collect();
        let mut cells = vec![];
        (0..size.x).for_each(|x| {
            (0..size.y).for_each(|y| {
                let top_left: VertexID = Self::vector_to_vertex_id(size, vec2!(x, y));
                let top_right: VertexID = Self::vector_to_vertex_id(size, vec2!(x + 1, y));
                let bottom_left: VertexID = Self::vector_to_vertex_id(size, vec2!(x, y + 1));
                let bottom_right: VertexID = Self::vector_to_vertex_id(size, vec2!(x, y) + 1);
                let mut top_edge: Option<EdgeRef> = None;
                let mut bottom_edge: Option<EdgeRef> = None;
                let mut left_edge: Option<EdgeRef> = None;
                let mut right_edge: Option<EdgeRef> = None;
                edges.iter().for_each(|x| {
                    let edge = &x.read().unwrap();
                    if edge.connects_to(top_left) {
                        if edge.connects_to(top_right) {
                            top_edge = Some(x.clone());
                        } else if edge.connects_to(bottom_left) {
                            left_edge = Some(x.clone());
                        }
                    } else if edge.connects_to(bottom_right) {
                        if edge.connects_to(top_right) {
                            right_edge = Some(x.clone());
                        } else if edge.connects_to(bottom_left) {
                            bottom_edge = Some(x.clone());
                        }
                    }
                });
                cells.push(Cell::new(vec![
                    top_edge.unwrap(),
                    right_edge.unwrap(),
                    bottom_edge.unwrap(),
                    left_edge.unwrap()
                ]));
            })
        });
        let cells = cells.into_iter().map(|x| Rc::new(RwLock::new(x))).collect();
        Grid {
            size,
            vertices: vec![Vertex::None; (size + vec2!(1)).area()],
            edges,
            cells,
        }
    }

    fn vector_to_vertex_id(size: GridVector2, vector: GridVector2) -> VertexID {
        vector.x + (size.x + 1) * vector.y
    }

    /// Gets the size of the grid, measured in square grid cells.
    /// For example:
    /// ```plaintext
    /// +---+---+---+
    /// |   |   |   |
    /// +---+---+---+
    /// |   |   |   |
    /// +---+---+---+
    /// ```
    /// This grid has a size of 3x2.
    pub fn get_size(&self) -> GridVector2 {
        self.size
    }

    /// Checks whether a vertex exists.
    pub fn vertex_exists(&self, id: VertexID) -> bool {
        id < (self.size + vec2!(1)).sum()
    }

    /// Returns a copy of the vertex at the given position, if it exists.
    pub fn get_vertex_type(&self, id: VertexID) -> Option<Vertex> {
        if self.vertex_exists(id) {
            Some(self.vertices[id])
        } else {
            None
        }
    }

    pub fn get_vertex_position(&self, id: VertexID) -> Option<GridVector2> {
        if self.vertex_exists(id) {
            let h_vert_count = self.size.x + 1;
            Some(vec2!(
                id % h_vert_count,
                id / h_vert_count // Integer division truncates aka rounds towards zero
            ))
        } else {
            None
        }
    }

    /// Returns an iterator that iterates over copies of the edges connected to the given vertex.
    pub fn get_edges_connected_to_vertex_iter(
        &self,
        id: VertexID,
    ) -> impl Iterator<Item = EdgeRef> + '_ {
        self.edges
            .iter()
            .cloned()
            .filter(move |edge| edge.read().unwrap().connects_to(id))
    }

    /// Returns a vector containing copies of the edges connected to the given vertex.
    pub fn get_edges_connected_to_vertex(&self, id: VertexID) -> Vec<EdgeRef> {
        self.get_edges_connected_to_vertex_iter(id).collect()
    }

    /// Gets a vector of the cells that border this cell.
    /// Assumes the cell's edges can be read.
    pub fn get_cells_around_cell(&self, cell: CellRef) -> Vec<CellRef> {
        let mut cells: Vec<CellRef> = cell
            .read()
            .unwrap()
            .get_edges()
            .iter()
            .filter_map(|e: &EdgeRef| {
                self.cells
                    .iter()
                    .find(|c: &&CellRef| {
                        c.read().unwrap().has_edge(&e) && !CellRef::ptr_eq(c, &cell)
                    })
                    .map(|x: &CellRef| x.clone())
            })
            .collect();
        cells.sort_by(|a: &CellRef, b: &CellRef| CellRef::as_ptr(a).cmp(&CellRef::as_ptr(b)));
        cells.dedup_by(|a: &mut CellRef, b: &mut CellRef| CellRef::ptr_eq(&a, &b));
        cells
    }

    /// Gets a vector of the cells that border this cell.
    /// Assumes the cell's edges can be read.
    pub fn get_cell_on_the_other_side_of_edge(
        &self,
        cell: CellRef,
        edge: EdgeRef,
    ) -> Option<CellRef> {
        self.cells
            .iter()
            .find(|c: &&CellRef| c.read().unwrap().has_edge(&edge) && !CellRef::ptr_eq(c, &cell))
            .map(|x: &CellRef| x.clone())
    }

    pub fn floodfill_from_cell(&self, cell: CellRef) -> Vec<CellRef> {
        let mut flood_filled = vec![cell.clone()];
        let mut not_done = VecDeque::new();
        not_done.push_back(cell);
        while let Some(front) = not_done.pop_front() {
            let mut filtered = self
                .get_cells_around_cell(front)
                .into_iter()
                .filter(|c| flood_filled.iter().any(|f| CellRef::ptr_eq(&f, c)))
                .collect::<Vec<CellRef>>();
            filtered
                .sort_by(|a: &CellRef, b: &CellRef| CellRef::as_ptr(a).cmp(&CellRef::as_ptr(b)));
            filtered.dedup_by(|a: &mut CellRef, b: &mut CellRef| CellRef::ptr_eq(&a, &b));
            flood_filled.extend(filtered.clone());
            not_done.extend(filtered);
        }
        flood_filled
    }
}
