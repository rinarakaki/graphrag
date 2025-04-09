//! A module containing run and _create_node_position methods definitions.

use std::backtrace;

use log::error;
use rustworkx_core::petgraph::graph::UnGraph;

use crate::index::operations::layout_graph::typing::{
    GraphLayout,
    NodePosition,
};
use crate::index::typing::error_handler::ErrorHandlerFn;

// TODO: This could be handled more elegantly, like what columns to use
// for "size" or "cluster"
// We could also have a boolean to indicate to use node sizes or clusters

/// Run method definition.
pub fn run<N, E>(
    graph: UnGraph<N, E>,
    on_error: ErrorHandlerFn,
) -> GraphLayout {
    let mut node_clusters = Vec::new();
    let mut node_sizes = Vec::new();

    let nodes = graph.raw_nodes();

    for node in nodes.iter() {
        let cluster = node.get("cluster", node.get("community", -1));
        node_clusters.push(cluster);
        let size = node.get("degree", node.get("size", 0));
        node_sizes.push(size);
    }

    match get_zero_positions(
        nodes,
        if !node_clusters.is_empty() { node_clusters } else { None },
        if !node_sizes.is_empty() { node_sizes } else { None },
        false,
    ) {
        Ok(zero_positions) => zero_positions,
        Err(e) => {
            error!("Error running zero-position");
            on_error(e, backtrace::format_exc(), None);
            // Umap may fail due to input sparseness or memory pressure.
            // For now, in these cases, we'll just return a layout with all nodes at (0, 0)
            let mut result = Vec::<NodePosition>::new();
            for (i, node) in nodes.iter().enumerate() {
                let cluster = if !node_clusters.is_empty() { node_clusters[i] } else { 1 };
                result.push(
                    NodePosition {
                        x: 0,
                        y: 0,
                        label: nodes[i],
                        size: 0,
                        cluster: str(cluster),
                    }
                );
            }
            result
        }
    }
}

/// Project embedding vectors down to 2D/3D using UMAP.
pub fn get_zero_positions(
    node_labels: Vec<&str>,
    node_categories: Option<Vec<usize>>,
    node_sizes: Option<Vec<usize>>,
    three_d: bool, // = false,
) -> Vec<NodePosition> {
    let mut embedding_position_data = Vec::<NodePosition>::new();
    for (index, node_name) in node_labels.iter().enumerate() {
        let node_category = if node_categories.is_none(){ 1 } else { node_categories[index] };
        let node_size = if node_sizes.is_none() { 1 }  else { node_sizes[index] };

        if !three_d {
            embedding_position_data.push(
                NodePosition {
                    label: str(node_name),
                    x: 0,
                    y: 0,
                    cluster: str(int(node_category)),
                    size: int(node_size),
                }
            );
        } else {
            embedding_position_data.push(
                NodePosition {
                    label: str(node_name),
                    x: 0,
                    y: 0,
                    z: 0,
                    cluster: str(int(node_category)),
                    size: int(node_size),
                }
            );
        }
    }
    embedding_position_data
}
