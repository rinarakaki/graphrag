//! Utilities to generate graph embeddings.

use rustworkx_core::petgraph::graph::Graph;
use ndarray::Array;

/// Node embeddings class definition.
pub struct NodeEmbeddings {
    pub nodes: Vec<String>,
    pub embeddings: Array,
}

/// Generate node embeddings using Node2Vec.
pub fn embed_node2vec<N, E, Ty>(
    graph: Graph<N, E, Ty>,
    dimensions: usize,
    num_walks: usize,
    walk_length: usize,
    window_size: usize,
    iterations: usize,
    random_seed: usize,
) -> NodeEmbeddings {
    // NOTE: This::is done here to reduce the initial::time of the graphrag package
    use graspologic as gc;

    // generate embedding
    let lcc_tensors = gc.embed.node2vec_embed(
        graph=graph,
        dimensions=dimensions,
        window_size=window_size,
        iterations=iterations,
        num_walks=num_walks,
        walk_length=walk_length,
        random_seed=random_seed,
    );
    NodeEmbeddings {
        embeddings: lcc_tensors[0],
        nodes: lcc_tensors[1],
    }
}
