//! Path in the TVG


use crate::tvg::tvg_graph::TvgEdge;




#[derive(Debug,Clone)]
/// Struct for paths in the TVG. The `edges` contains a path of `TvgEdge`-s in order.
pub struct TvgPath {
   pub edges: Vec<TvgEdge>
}




impl TvgPath {
    /// Create new path
    pub fn new() -> Self {
        TvgPath {
            edges: Vec::new()
        }
    }

    /// Create a new path from an existing vector of `TvgEdge`-s
    pub fn from_vec(paths: Vec<TvgEdge>) -> Self {
        TvgPath {
            edges: paths
        }
    }
}