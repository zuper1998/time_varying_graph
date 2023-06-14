use crate::tvg::internals::IntervalTvgEdge;

use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize)]
pub struct JsonTvgData {
    pub(crate) nodes : Vec<String>,
    pub(crate) edges: Vec<JsonTvgEdge>

}
#[derive(Serialize,Deserialize)]
pub struct JsonTvgEdge{
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) start: f32,
    pub(crate) end: f32,
    pub(crate) data: Option<f32>
}
/// Data container for Tvg construction
#[derive(Debug)]
pub struct TvgData {
    pub start_node: String,
    pub end_node: String,

    pub interval: IntervalTvgEdge,
}