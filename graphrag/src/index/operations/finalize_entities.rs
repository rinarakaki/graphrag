//! All the steps to transform final entities.

use polars::prelude::{col, JoinArgs, JoinType, LazyFrame, UniqueKeepStrategy};
use uuid::Uuid;

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::models::embed_graph_config::EmbedGraphConfig;
use crate::data_model::schemas::ENTITIES_FINAL_COLUMNS;
use crate::index::operations::compute_degree::compute_degree;
use crate::index::operations::create_graph::create_graph;
use crate::index::operations::embed_graph::embed_graph::embed_graph;
use crate::index::operations::layout_graph::layout_graph::layout_graph;

/// All the steps to transform final entities.
pub fn finalize_entities(
    entities: LazyFrame,
    relationships: LazyFrame,
    callbacks: impl WorkflowCallbacks,
    embed_config: Option<EmbedGraphConfig>,
    layout_enabled: bool,
) -> LazyFrame {
    let graph = create_graph(relationships);
    let mut graph_embeddings = None;
    if let Some(embed_config) = embed_config {
        if embed_config.enabled {
            graph_embeddings = Some(embed_graph(graph, embed_config));
        }
    }
    let layout = layout_graph(
        graph,
        callbacks,
        layout_enabled,
        graph_embeddings,
    );
    let degrees = compute_degree(graph);
    let final_entities = entities
        .join(layout, [col("title")], [col("label")], JoinArgs::new(JoinType::Left))
        .join(degrees, [col("title")], [col("title")], JoinArgs::new(JoinType::Left))
        .unique(Some(vec!["title".into()]), UniqueKeepStrategy::First);
    let final_entities = final_entities[entities["title"].is_not_nan()];
    // disconnected nodes and those with no community even at level 0 can be missing degree
    final_entities["degree"] = final_entities["degree"].fillna(0).astype(int);
    final_entities["human_readable_id"] = final_entities.index;
    final_entities["id"] = final_entities["human_readable_id"].apply(|_x| Uuid::new_v4().to_string());
    final_entities[..][ENTITIES_FINAL_COLUMNS]
}
