
use std::fs::File;
use std::io::Write;

use petgraph::{Directed, Graph, Outgoing};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex};
use indexmap::IndexSet;
use allen_interval_algebra::interval::Interval;
use petgraph::prelude::{EdgeIndex, EdgeRef};
use serde::{Deserialize, Serialize};
use crate::tvg::internals::IntervalTvgEdge;
use crate::tvg::internals::IntervalTvgEdge::{BaseEdge, DataEdge};
use crate::tvg::serialization::{TvgData,JsonTvgData,JsonTvgEdge};
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
                self.add_path_to_channel(&visited, paths, neighbour,None)
            } else {
                let mut give_visited = visited.clone();
                give_visited.insert(neighbour);
                self.tvg_bfs(neighbour, give_visited, paths, search_function, max_depth);
            }
        }
    }

    /// Export the graph to json
    pub fn export_to_json(&self) -> String {
        let mut json_data: JsonTvgData = JsonTvgData{nodes: Vec::new(),edges: Vec::new()};
        for node_index in  self.graph.node_indices() {
            json_data.nodes.push(self.graph[node_index].to_string());
            for edge_ref in self.graph.edges(node_index){

                for edge in edge_ref.weight() {

                    match edge {
                        BaseEdge(interval) => {
                            json_data.edges.push(JsonTvgEdge{
                                from: self.graph[edge_ref.source()].clone(),
                                to: self.graph[edge_ref.target()].clone(),
                                start: interval.start,
                                end: interval.end,
                                data: None
                            })
                        }
                        DataEdge(interval,data) => {
                            json_data.edges.push(JsonTvgEdge{
                                from: self.graph[edge_ref.source()].clone(),
                                to: self.graph[edge_ref.target()].clone(),
                                start: interval.start,
                                end: interval.end,
                                data: Some(*data)
                            })
                        }


                    }



                }
            }

        }
        serde_json::to_string(&json_data).unwrap()
    }




    fn add_path_to_channel(&self, visited: &IndexSet<NodeIndex>, paths: &crossbeam_channel::Sender<(String, TvgPath)>,
                           neighbour: NodeIndex, bidirectional:Option<bool> ) {
        let mut edges: Vec<TvgEdge> = Vec::new();
        let node_path: Vec<NodeIndex> = visited
            .iter()
            .cloned()
            .chain(Some(neighbour))
            .collect::<Vec<NodeIndex>>();


        for pair in node_path.windows(2) {
            let mut i_edges: Vec<IntervalTvgEdge> = Vec::new();
            self.add_interval_edges_from_to(pair[0], pair[1], &mut i_edges);
            if bidirectional.unwrap_or(false) {
                self.add_interval_edges_from_to(pair[1], pair[0], &mut i_edges);
            }
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





