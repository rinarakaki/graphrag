//! A module containing layout_graph, _run_layout and _apply_layout_to_graph methods definition.

use rustworkx_core::petgraph::graph::UnGraph;
use polars::prelude::LazyFrame;

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::index::operations::embed_graph::typing::NodeEmbeddings;
use crate::index::operations::layout_graph::typing::GraphLayout;

/**
Apply a layout algorithm to a nx.Graph. The method returns a LazyFrame containing the node positions.

## Usage
```yaml
args:
    graph: The nx.Graph to layout
    embeddings: Embeddings for each node in the graph
    strategy: <strategy config> # See strategies section below
```

## Strategies
The layout graph verb uses a strategy to layout the graph. The strategy is a json object which defines the strategy to use. The following strategies are available:

### umap
This strategy uses the umap algorithm to layout a graph. The strategy config is as follows:
```yaml
strategy:
    type: umap
    n_neighbors: 5 # Optional, The number of neighbors to use for the umap algorithm, default: 5
    min_dist: 0.75 # Optional, The min distance to use for the umap algorithm, default: 0.75
```
 */
pub fn layout_graph<N, E>(
    graph: UnGraph<N, E>,
    callbacks: impl WorkflowCallbacks,
    enabled: bool,
    embeddings: Option<NodeEmbeddings>,
) -> LazyFrame {
    let layout = _run_layout(
        graph,
        enabled,
        embeddings.unwrap_or_default(),
        callbacks,
    );

    let layout_df = LazyFrame::from(layout);
    layout_df[..][["label", "x", "y", "size"]]
}

fn _run_layout<N, E>(
    graph: UnGraph<N, E>,
    enabled: bool,
    embeddings: NodeEmbeddings,
    callbacks: impl WorkflowCallbacks,
) -> GraphLayout {
    if enabled {
        use crate::index::operations::layout_graph::umap::run as run_umap;

        return run_umap(
            graph,
            embeddings,
            |e, stack, d| callbacks.error("Error in Umap", e, stack, d),
        )
    }
    use crate::index::operations::layout_graph::zero::run as run_zero;

    run_zero(
        graph,
        |e, stack, d| callbacks.error("Error in Zero", e, stack, d),
    )
}
