//! Library for creating and managing time-varying graphs, currently in the works.
//!
//! # Example
//!
//! ```rust
//!
//!  use time_varying_graph::tvg::{Tvg};
//!
//!  let data = r#"
//!        {
//!          "nodes": [
//!            "Node1",
//!            "Node2"
//!          ],
//!          "edges": [
//!            {
//!              "from": "Node1",
//!              "to": "Node2",
//!              "start" : 0.0,
//!              "end" : 1.0,
//!              "data": null
//!            }
//!          ]
//!        }"#;
//!
//!
//! // Create a time-varying graph:
//! let mut time_varying_graph = Tvg::new();
//! // Add data from json (recommended)
//! time_varying_graph.add_edges_from_json(data.to_string());
//!
//!
//!
//! ```
//!
//!


use crate::tvg::{IntervalTvgEdge};
pub mod tvg;
pub mod tvg_path;

#[cfg(test)]
mod tests {
    use allen_interval_algebra::interval::Interval;
    use indexmap::IndexSet;
    use petgraph::graph::NodeIndex;
    use crate::tvg::{IntervalTvgEdge, Tvg, TvgData};

    #[test]
    fn test_json_load() {
        let data = r#"
        {
          "nodes": [
            "Node1",
            "Node2"
          ],
          "edges": [
            {
              "from": "Node1",
              "to": "Node2",
              "start" : 0.0,
              "end" : 1.0,
              "data": null
            }
          ]
        }"#;
        let mut tvg = Tvg::new();
        tvg.add_edges_from_json(data.to_string());

        assert_ne!(tvg.find_node("Node1".to_string()), None);
        assert_ne!(tvg.find_edge("Node1".to_string(),"Node2".to_string()), None);



    }

    #[test]
    fn test_tvg_data_load() {
        let mut tvg = Tvg::new();
        let mut v: Vec<TvgData> = Vec::new();

        v.push(TvgData {
            start_node: String::from("N1"),
            end_node:  String::from("N2"),
            interval: IntervalTvgEdge::DataEdge {
                0: Interval {
                    start: 0.0,
                    end: 1.0,
                },
                1: 52.0,
            },
        });
        v.push(TvgData {
            start_node: String::from("N1"),
            end_node:  String::from("N3"),
            interval: IntervalTvgEdge::DataEdge {
                0: Interval {
                    start: 0.0,
                    end: 3.0,
                },
                1: 52.1,
            },
        });

        tvg.add_edges_from_data(v);

        assert_ne!(tvg.find_edge(String::from("N1"), String::from("N3")), None);
        assert_ne!(tvg.find_edge(String::from("N1"), String::from("N2")), None);
        assert_ne!(tvg.find_node(String::from("N1")), None);
        assert_ne!(tvg.find_node(String::from("N2")), None);
        assert_ne!(tvg.find_node(String::from("N3")), None);


    }

    #[test]
    fn tvg_bfs_test() {
        use crossbeam_channel::bounded;


        let data = r#"
            {
              "nodes": [
                "Node1",
                "Node2",
                "Node3",
                "Node4"
              ],
              "edges": [
                {
                  "from": "Node1",
                  "to": "Node2",
                  "start" : 0.0,
                  "end" : 1.0,
                  "data": null
                },
                {
                  "from": "Node2",
                  "to": "Node3",
                  "start" : 0.0,
                  "end" : 1.0,
                  "data": null
                },
                {
                  "from": "Node2",
                  "to": "Node4",
                  "start" : 0.0,
                  "end" : 1.0,
                  "data": null
                },
                {
                  "from": "Node1",
                  "to": "Node2",
                  "start" : 0.0,
                  "end" : 1.0,
                  "data": null
                }
              ]
            }"#;
        let mut tvg = Tvg::new();
        tvg.add_edges_from_json(data.to_string());
        let start = tvg.find_node("Node1".to_string()).unwrap();
        let visited: IndexSet<NodeIndex> = IndexSet::from_iter(Some(start));

        let (sender, receiver) = bounded(50);

        let _ = std::thread::spawn(move || {
            tvg.tvg_bfs(start, visited, &sender, |data: &String| {data.contains("Node4")}, None  );
        });

        while let Ok((node_name, path)) = receiver.recv() {
            assert!(node_name.eq("Node4"))
        }




    }





}