//! Time varying graph


use std::fs::File;
use std::io::Write;

use log::error;
use petgraph::{Directed, Graph, Outgoing};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex};
use indexmap::IndexSet;
use allen_interval_algebra::interval::Interval;
use petgraph::prelude::EdgeIndex;
use serde::Deserialize;
use crate::IntervalTvgEdge::*;
use crate::tvg_path::TvgPath;

/// Struct for containing multiple `IntervalTvgEdge`-s
#[derive(Debug, Clone)]
pub struct TvgEdge {
    pub edges: Vec<IntervalTvgEdge>,
}


impl TvgEdge {
    /// Create new TvgEdge.
    pub fn new() -> Self {
        TvgEdge {
            edges: Vec::new()
        }
    }

    /// Create TvgEdge from a vector of `IntervalTvgEdge`-s
    pub fn from_vec(edges: Vec<IntervalTvgEdge>) -> Self {
        TvgEdge {
            edges
        }
    }
}



/// Enum for the time-varying graph edge types. `BaseEdge` is for simple edges compromised from only
/// an interval, while `DataEdge` should be used in cases where there is also some data associated
/// with an interval. For example data sent over time.
#[derive(Debug, Clone, Copy)]
pub enum IntervalTvgEdge {
    /// Simple interval edge without data
    BaseEdge(Interval<f32>),
    /// Interval edge with float data
    DataEdge(Interval<f32>, f32),
}

impl IntervalTvgEdge {
    /// `eq` function for the IntervalTvgEdge, in case of comparing two different edges it will panic!
    pub fn eq(self, other: &IntervalTvgEdge) -> bool {
        return match (&self, other) {
            (BaseEdge(interval), BaseEdge(other_interval)) => {
                interval.start == other_interval.start && interval.end == other_interval.end
            }

            (DataEdge(interval, _data), DataEdge(other_interval, _other_data)) => {
                interval.start == other_interval.start && interval.end == other_interval.end
            }

            _ => {
                error!("Edge types are not matching! self: {:?} other: {:?}",self,other);
                panic!("Edge types are not matching! self: {:?} other: {:?}", self, other);
            }
        };
    }
}


/// A time varying-graph (TVG) representation
/// Each edge contains connection times as intervals and may contain other data (eg: sent bits).
#[derive(Debug)]
pub struct Tvg {
    graph: Graph<String, Vec<IntervalTvgEdge>, Directed>,
}


impl Tvg {
    /// Create new graph.
    pub fn new() -> Self {
        Tvg {
            graph: Graph::new(),
        }
    }

    /// Add edges from json. The format should be something like this:
    ///
    /// ```json
    /// {
    ///     "nodes" : [
    ///         "NodeName"
    ///     ],
    ///     "edges" : [{
    ///         "from" : String,
    ///         "to" : String,
    ///         "start" : f32,
    ///         "end" : f32,
    ///         "data" : Option<f32>
    ///     }]
    /// }
    /// ```
    pub fn add_edges_from_json(&mut self ,data: String) {
        #[derive(Deserialize)]
        struct JsonTvgData {
            nodes : Vec<String>,
            edges: Vec<JsonTvgEdge>

        }
        #[derive(Deserialize)]
        struct JsonTvgEdge{
            from: String,
            to: String,
            start: f32,
            end: f32,
            data: Option<f32>
        }


        let values : JsonTvgData = serde_json::from_str(&data).unwrap();

        for node in values.nodes {
            self.add_node_no_dup(&node);
        }
        for edge in values.edges {
            let from_index = self.find_node(edge.from).unwrap();
            let to_index = self.find_node(edge.to).unwrap();
            if let Some(data) = edge.data {
                self.add_edge_no_dup(from_index,to_index,DataEdge(Interval::new(edge.start,edge.end) ,data))
            } else {
                self.add_edge_no_dup(from_index,to_index,BaseEdge(Interval::new(edge.start,edge.end)))
            }
        }



    }


    /// Add edges from a vector of `TvgData`
    pub fn add_edges_from_data(&mut self, edges: Vec<TvgData>) {
        for edge in edges {
            let start_index = self.add_node_no_dup(&edge.start_node);
            let end_index = self.add_node_no_dup(&edge.end_node);


            self.add_edge_no_dup(start_index, end_index, edge.interval);
        }
    }

    /// Print the whole graph to the dot format.
    pub fn print_to_dot(&self) {
        let mut file = File::create("tvg.dot").unwrap();
        let to_write = format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));
        //println!("{:?}", to_write);
        file.write(to_write.as_bytes()).unwrap();
    }


    /// Find a node based on its name.
    pub fn find_node(&self, name: String) -> Option<NodeIndex> {
        self.graph.node_indices().find(|index| {
            let node: String = self.graph[*index].to_string();
            node.eq(&name)
        }
        )
    }

    /// Find an edge based on its start and end node.
    pub fn find_edge(&self, start_name: String,  end_name: String) -> Option<EdgeIndex> {
        self.graph.find_edge(self.find_node(start_name)?,self.find_node(end_name)?)
    }


    /// This is an *interesting* function, because in some cases the best path maybe totally unrelated
    /// from the number of steps in the TVG there might be need to just walk trough all the possible
    /// paths between two points. This function support setting a `target`  function. The function
    /// will send all the paths to the supplied crossbeam_channel.
    pub fn tvg_bfs(&self, start: NodeIndex, visited: IndexSet<NodeIndex>, paths: &crossbeam_channel::Sender<(String, TvgPath)>,
                   search_function: fn(&String) -> bool, max_depth: Option<usize>) {

        if visited.len() > max_depth.unwrap_or(3) { return; }


        for neighbour in self.graph.neighbors_directed(start, Outgoing) {
            if visited.contains(&neighbour) {
                continue;
            }


            if search_function(self.graph.node_weight(neighbour).unwrap()) {
                self.add_path_to_channel(&visited, paths, neighbour)
            } else {
                let mut give_visited = visited.clone();
                give_visited.insert(neighbour);
                self.tvg_bfs(neighbour, give_visited, paths, search_function, max_depth);
            }
        }
    }



    fn add_path_to_channel(&self, visited: &IndexSet<NodeIndex>, paths: &crossbeam_channel::Sender<(String, TvgPath)>, neighbour: NodeIndex) {
        let mut edges: Vec<TvgEdge> = Vec::new();
        let node_path: Vec<NodeIndex> = visited
            .iter()
            .cloned()
            .chain(Some(neighbour))
            .collect::<Vec<NodeIndex>>();


        for pair in node_path.windows(2) {
            let mut i_edges: Vec<IntervalTvgEdge> = Vec::new();
            self.add_interval_edges_from_to(pair[0], pair[1], &mut i_edges);
            self.add_interval_edges_from_to(pair[1], pair[0], &mut i_edges);
            edges.push(TvgEdge::from_vec(i_edges));
        }

        paths.send((self.graph[neighbour].to_string(), TvgPath::from_vec(edges))).unwrap();
    }

    fn add_interval_edges_from_to(&self, from: NodeIndex, to: NodeIndex, i_edges: &mut Vec<IntervalTvgEdge>) {
        let edge_index = self.graph.find_edge(from, to).unwrap();
        let tvg_edges = &self.graph[edge_index];
        for e in tvg_edges {
            i_edges.push(e.clone());
        }
    }



    fn add_edge_no_dup(&mut self, start_node: NodeIndex, end_node: NodeIndex, edge: IntervalTvgEdge) {
        if let Some(index) = self.graph.find_edge(start_node, end_node)
        {
            match edge {
                BaseEdge(interval) => {
                    self.graph.edge_weight_mut(index).unwrap().push(BaseEdge(interval));
                }
                DataEdge(interval, data) => {
                    self.graph.edge_weight_mut(index).unwrap().push(DataEdge(interval, data));
                }
            }
        } else {
            match edge {
                BaseEdge(interval) => {
                    self.graph.add_edge(start_node, end_node, vec![BaseEdge(interval)]);
                }
                DataEdge(interval, data) => {
                    self.graph.add_edge(start_node, end_node, vec![DataEdge(interval, data)]);
                }
            }
        }
    }

    fn add_node_no_dup(&mut self, data: &String) -> NodeIndex {
        if let Some(ret_index) = self.graph.node_indices().find(|index| self.graph[*index].eq(data)) {
            return ret_index;
        }

        self.graph.add_node(data.clone())
    }
}


/// Data container for Tvg construction
#[derive(Debug)]
pub struct TvgData {
    pub start_node: String,
    pub end_node: String,

    pub interval: IntervalTvgEdge,
}


