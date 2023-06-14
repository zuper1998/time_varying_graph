//! Time varying graph
//! It can be used in the following way:
//! ```rust
//! use crossbeam_channel::bounded;
//! use time_varying_graph::tvg::{Tvg};
//! use indexmap::IndexSet;
//! use petgraph::graph::NodeIndex;
//! use time_varying_graph::tvg::tvg_graph::Tvg;
//!         let data = r#"
//!             {
//!               "nodes": [
//!                 "Node1",
//!                 "Node2",
//!                 "Node3",
//!                 "Node4"
//!               ],
//!               "edges": [
//!                 {
//!                   "from": "Node1",
//!                   "to": "Node2",
//!                   "start" : 0.0,
//!                   "end" : 1.0,
//!                   "data": null
//!                 },
//!                 {
//!                   "from": "Node2",
//!                   "to": "Node3",
//!                   "start" : 0.0,
//!                   "end" : 1.0,
//!                   "data": null
//!                 },
//!                 {
//!                   "from": "Node2",
//!                   "to": "Node4",
//!                   "start" : 0.0,
//!                   "end" : 1.0,
//!                   "data": null
//!                 },
//!                 {
//!                   "from": "Node1",
//!                   "to": "Node2",
//!                   "start" : 0.0,
//!                   "end" : 1.0,
//!                   "data": null
//!                 }
//!               ]
//!             }"#;
//!         let mut tvg = Tvg::new();
//!         tvg.add_edges_from_json(data.to_string());
//!         let start = tvg.find_node("Node1".to_string()).unwrap();
//!         let visited: IndexSet<NodeIndex> = IndexSet::from_iter(Some(start));
//!
//!         // We use a bounded container otherwise we would run out of memory in case of bigger graphs
//!         let (sender, receiver) = bounded(50);
//!
//!         // Run the bfs in a thread to be able to instantly use its output
//!         let _ = std::thread::spawn(move || {
//!             tvg.tvg_bfs(start, visited, &sender, |data: &String| {data.contains("Node4")}, None  );
//!         });
//!
//!         while let Ok((node_name, path)) = receiver.recv() {
//!             assert!(node_name.eq("Node4"))
//!             // Do some other stuff with the path and the node.
//!         }
//!
//!
//!
//!
//! ```
//!
//!
//!
pub mod tvg_graph;
pub mod serialization;
pub mod internals;