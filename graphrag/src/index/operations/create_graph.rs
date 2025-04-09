//! A module containing create_graph definition.

use rustworkx_core::petgraph::graph::UnGraph;
use polars::prelude::LazyFrame;

/// Create a networkx graph from nodes and edges dataframes.
pub fn create_graph(
    edges: LazyFrame,
    edge_attr: Option<Vec<String>>,  //  | int
    nodes: Option<LazyFrame>,
    node_id: &str, // = "title",
) -> UnGraph<LazyFrame, LazyFrame> {
    let graph = nx::from_pandas_edgelist(edges, edge_attr=edge_attr);

    if let Some(nodes) = nodes {
        graph.add_nodes_from((n, dict(d)) for n, d in nodes.iterrows());
    }

    graph
}
