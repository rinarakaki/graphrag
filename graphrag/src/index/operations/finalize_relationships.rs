//! All the steps to transform final relationships.

use uuid::Uuid;

use polars::prelude::{LazyFrame, UniqueKeepStrategy};

use crate::data_model::schemas::RELATIONSHIPS_FINAL_COLUMNS;
use crate::index::operations::compute_degree::compute_degree;
use crate::index::operations::compute_edge_combined_degree::compute_edge_combined_degree;
use crate::index::operations::create_graph::create_graph;

/// All the steps to transform final relationships.
pub fn finalize_relationships(
    relationships: LazyFrame,
) -> LazyFrame {
    let graph = create_graph(relationships);
    let degrees = compute_degree(graph);

    let mut final_relationships = relationships.unique(
        Some(vec!["source", "target"].iter().map(|s| s.to_string()).collect()),
        UniqueKeepStrategy::First,
    );
    final_relationships["combined_degree"] = compute_edge_combined_degree(
        final_relationships,
        degrees,
        "title",
        "degree",
        "source",
        "target",
    );

    final_relationships["human_readable_id"] = final_relationships.index;
    final_relationships["id"] = final_relationships["human_readable_id"].apply(
        |_x| Uuid::new_v4()
    );

    final_relationships[..][RELATIONSHIPS_FINAL_COLUMNS]
}
