use crate::tvg::{IntervalTvgEdge};
pub mod tvg;
pub mod tvg_path;

#[cfg(test)]
mod tests {
    use allen_interval_algebra::interval::Interval;
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




}