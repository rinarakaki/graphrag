//! A module containing embed_graph and run_embeddings methods definition.

use std::iter::zip;

use rustworkx_core::petgraph::graph::UnGraph;

use crate::config::models::embed_graph_config::EmbedGraphConfig;
use crate::index::operations::embed_graph::embed_node2vec::embed_node2vec;
use crate::index::operations::embed_graph::typing::NodeEmbeddings;
use crate::index::utils::stable_lcc::stable_largest_connected_component;

/**
Embed a graph into a vector space using node2vec. The graph is expected to be in nx.Graph format. The operation outputs a mapping between node name and vector.

## Usage
```yaml
dimensions: 1536 # Optional, The number of dimensions to use for the embedding, default: 1536
num_walks: 10 # Optional, The number of walks to use for the embedding, default: 10
walk_length: 40 # Optional, The walk length to use for the embedding, default: 40
window_size: 2 # Optional, The window size to use for the embedding, default: 2
iterations: 3 # Optional, The number of iterations to use for the embedding, default: 3
random_seed: 86 # Optional, The random seed to use for the embedding, default: 86
```
 */
pub fn embed_graph<N, E>(
    mut graph: UnGraph<N, E>,
    config: EmbedGraphConfig,
) -> NodeEmbeddings {
    if config.use_lcc {
        graph = stable_largest_connected_component(graph);
    }

    // create graph embedding using node2vec
    let embeddings = embed_node2vec(
        graph,
        config.dimensions,
        config.num_walks,
        config.walk_length,
        config.window_size,
        config.iterations,
        config.random_seed,
    );

    let pairs = zip(embeddings.nodes, embeddings.embeddings.tolist());
    pairs.sort_by(|a, b| a[0].cpm(b[0]));

    pairs
}
