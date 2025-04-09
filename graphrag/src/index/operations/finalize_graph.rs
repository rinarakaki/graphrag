//! A module containing create_graph definition.

use rustworkx_core::petgraph::graph::UnGraph;
use polars::prelude::LazyFrame;

/// Create a networkx graph from nodes and edges dataframes.
pub fn create_graph<N, E>(
    edges: LazyFrame,
    edge_attr: Option<Vec<str | int>>,
    nodes: Option<LazyFrame>,
    node_id: str,  // = "title"
) -> UnGraph<N, E> {
    let graph = nx.from_pandas_edgelist(edges, edge_attr);

    if let Some(nodes) = nodes {
        for (n, d) in nodes.iter_rows() {
            graph.add_nodes_from((n, dict(d)) );
        }
    }

    graph
}
