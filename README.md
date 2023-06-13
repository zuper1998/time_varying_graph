# time_varying_graph

## Description 

The main purpose of this library is to provide a simple library for interacting with time varying graphs.

## Use 



```rust
use time_varying_graph::tvg::{Tvg};



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


    // Create a time-varying graph:
    let mut time_varying_graph = Tvg::new();
    // Add data from json (recommended)
    time_varying_graph.add_edges_from_json(data.to_string());
```
