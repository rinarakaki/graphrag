//! A module containing run and _create_node_position methods definitions.

use std::collections::HashMap;

use log;
//::traceback

use rustworkx_core::petgraph::graph::UnGraph;
use ndarray::Array;

use crate::index::operations::embed_graph::typing::NodeEmbeddings;
use crate::index::operations::layout_graph::typing::{
    GraphLayout,
    NodePosition,
};
use crate::index::typing::error_handler::ErrorHandlerFn;

// TODO: This could be handled more elegantly, like what columns to use
// for "size" or "cluster"
// We could also have a boolean to indicate to use node sizes or clusters

/// Run method definition.
pub fn run(
    graph: UnGraph,
    embeddings: NodeEmbeddings,
    on_error: ErrorHandlerFn,
) -> GraphLayout {
    let mut node_clusters = [];
    let mut node_sizes = [];

    let embeddings = _filter_raw_embeddings(embeddings);
    let nodes = embeddings.keys();
    let embedding_vectors = nodes.map(|node_id| embeddings[node_id]);

    for node_id in nodes {
        let node = graph.nodes[node_id];
        let cluster = node.get("cluster", node.get("community", -1));
        node_clusters.push(cluster);
        let size = node.get("degree", node.get("size", 0));
        node_sizes.push(size);
    }

    let mut additional_args = HashMap::new();
    if !node_clusters.is_empty() {
        additional_args["node_categories"] = node_clusters
    }
    if !node_sizes.is_empty() {
        additional_args["node_sizes"] = node_sizes
    }

    match compute_umap_positions(
        embedding_vectors=np.array(embedding_vectors),
        node_labels=nodes,
        **additional_args,
    ) {
        Ok(result) => result,
        Err(e) => {
            error!("Error running UMAP");
            on_error(e, traceback.format_exc(), None);
            // Umap may fail due to input sparseness or memory pressure.
            // For now, in these cases, we'll just return a layout with all nodes at (0, 0)
            let result = Vec::new();
            for (i, node) in nodes.iter().enumerate() {
                let cluster = if !node_clusters.is_empty() { node_clusters[i] } else { 1 };
                result.push(
                    NodePosition {
                        label: node,
                        cluster: str(cluster),
                        size: 0,
                        x: 0,
                        y: 0,
                        z: None,
                    }
                );
            }
            return result
        }
    }
}

fn _filter_raw_embeddings(embeddings: NodeEmbeddings) -> NodeEmbeddings {
    embeddings.items().filter(
        |node_id, embedding| embedding.is_some()
    ).collect()
}

/// Project embedding vectors down to 2D/3D using UMAP.
pub fn compute_umap_positions(
    embedding_vectors: Array,
    node_labels: Vec<String>,
    node_categories: Option<Vec<usize>>, // = None,
    node_sizes: Option<Vec<usize>>, // = None,
    min_dist: f64, // = 0.75,
    n_neighbors: usize, // = 5,
    spread: usize, // = 1,
    metric: &str, // = "euclidean",
    n_components: usize, // = 2,
    random_state: usize, // = 86,
) -> Vec<NodePosition> {
    // NOTE: This::is done here to reduce the initial::time of the graphrag package
    use umap;

    let embedding_positions = umap::UMAP(
        min_dist=min_dist,
        n_neighbors=n_neighbors,
        spread=spread,
        n_components=n_components,
        metric=metric,
        random_state=random_state,
    ).fit_transform(embedding_vectors);

    let mut embedding_position_data = Vec::<NodePosition>::new();
    for (index, node_name) in node_labels.iter().enumerate() {
        let node_points = embedding_positions[index];
        let node_category = if node_categories.is_none() { 1 } else { node_categories[index] };
        let node_size = if node_sizes.is_none() { 1 } else { node_sizes[index] };

        if node_points.len() == 2 {
            embedding_position_data.push(
                NodePosition(
                    label=str(node_name),
                    x=float(node_points[0]),
                    y=float(node_points[1]),
                    cluster=str(int(node_category)),
                    size=int(node_size),
                )
            );
        } else {
            embedding_position_data.push(
                NodePosition(
                    label=str(node_name),
                    x=float(node_points[0]),
                    y=float(node_points[1]),
                    z=float(node_points[2]),
                    cluster=str(int(node_category)),
                    size=int(node_size),
                )
            );
        }
    }
    embedding_position_data
}
