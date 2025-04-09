//! A module containing create_graph definition.

use rustworkx_core::petgraph::graph::UnGraph;
use polars::prelude::LazyFrame;

/// Create a new LazyFrame with the degree of each node in the graph.
pub fn compute_degree<N, E>(graph: UnGraph<N, E>) -> LazyFrame {
    LazyFrame::from([
        {"title": node, "degree": int(degree)}
        for (node, degree) in graph.degree
    ])
}
