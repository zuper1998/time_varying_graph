use crate::tvg::TvgEdge;


#[derive(Debug,Clone)]
pub struct TvgPath {
   pub edges: Vec<TvgEdge>
}




impl TvgPath {
    pub fn new() -> Self {
        TvgPath {
            edges: Vec::new()
        }
    }
    pub fn from_vec(paths: Vec<TvgEdge>) -> Self {
        TvgPath {
            edges: paths
        }
    }
}