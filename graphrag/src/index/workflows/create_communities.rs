//! A module containing run_workflow method definition.

// from datetime::datetime, timezone
// from uuid::uuid4

use ndarray::Array;
use polars::prelude::{LazyFrame, col, JoinArgs, JoinType};

use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::data_model::schemas::COMMUNITIES_FINAL_COLUMNS;
use crate::index::operations::cluster_graph::cluster_graph;
use crate::index::operations::create_graph::create_graph;
use crate::index::typing::context::PipelineRunContext;
use crate::index::typing::workflow::WorkflowFunctionOutput;
use crate::utils::storage::{load_table_from_storage, write_table_to_storage};

/// All the steps to transform final communities.
pub async fn run_workflow(
    config: GraphRagConfig,
    context: PipelineRunContext,
) -> WorkflowFunctionOutput {
    let entities = load_table_from_storage("entities", context.storage).await;
    let relationships = load_table_from_storage("relationships", context.storage).await;

    let max_cluster_size = config.cluster_graph.max_cluster_size;
    let use_lcc = config.cluster_graph.use_lcc;
    let seed = config.cluster_graph.seed;

    let output = create_communities(
        entities,
        relationships,
        max_cluster_size,
        use_lcc,
        Some(seed),
    );

    write_table_to_storage(output, "communities", context.storage).await;

    WorkflowFunctionOutput {
        result: output
    }
}

/// All the steps to transform final communities.
pub fn create_communities(
    entities: LazyFrame,
    relationships: LazyFrame,
    max_cluster_size: usize,
    use_lcc: bool,
    seed: Option<usize>,
) -> LazyFrame {
    let graph = create_graph(relationships);

    let clusters = cluster_graph(
        graph,
        max_cluster_size,
        use_lcc,
        seed=seed,
    );

    let mut communities = LazyFrame::from(
        clusters, columns=pd.Index(["level", "community", "parent", "title"])
    ).explode("title");
    communities["community"] = communities["community"].astype(int);

    // aggregate entity ids for each community
    let entity_ids = communities.join(entities, [col("title")], [col("title")], JoinArgs::new(JoinType::Inner));
    let entity_ids = (
        entity_ids.group_by("community").agg(entity_ids=("id", list)).reset_index()
    );

    // aggregate relationships ids for each community
    // these are limited to only those where the source and target are in the same community
    let max_level = communities["level"].max();
    let all_grouped = LazyFrame::from(
        columns=["community", "level", "relationship_ids", "text_unit_ids"]
    );
    for level in 0..(max_level + 1) {
        let communities_at_level = communities.loc[communities["level"] == level];
        let sources = relationships.join(
            communities_at_level, [col("source")], [col("title")], JoinArgs::new(JoinType::Inner)
        );
        let targets = sources.join(
            communities_at_level, [col("target")], [col("title")], JoinArgs::new(JoinType::Inner)
        );
        let matched = targets.loc[targets["community_x"] == targets["community_y"]];
        let text_units = matched.explode("text_unit_ids");
        let grouped = text_units.group_by(["community_x", "level_x", "parent_x"])
        .agg(relationship_ids=("id", list), text_unit_ids=("text_unit_ids", list))
        .reset_index();
        let grouped.rename(
            columns={
                "community_x": "community",
                "level_x": "level",
                "parent_x": "parent",
            },
            inplace=True,
        );
        let all_grouped = pd.concat([
            all_grouped,
            grouped[..][
                ["community", "level", "parent", "relationship_ids", "text_unit_ids"]
            ],
        ]);
    }

    // deduplicate the lists
    all_grouped["relationship_ids"] = all_grouped["relationship_ids"].apply(
        |x| sorted(set(x))
    );
    all_grouped["text_unit_ids"] = all_grouped["text_unit_ids"].apply(
        |x| sorted(set(x))
    );

    // join it all up and add some new fields
    let mut final_communities = all_grouped.join(entity_ids, [col("community")], [col("community")], JoinArgs::new(JoinType::Inner));
    final_communities["id"] = [str(uuid4()) for _ in range(len(final_communities))];
    final_communities["human_readable_id"] = final_communities["community"];
    final_communities["title"] = "Community " + final_communities["community"].astype(
        str
    );
    final_communities["parent"] = final_communities["parent"].astype(int);
    // collect the children so we have a tree going both ways
    let parent_grouped = final_communities.group_by("parent").agg(children=("community", "unique"));
    let final_communities = final_communities.join(
        parent_grouped,
        [col("community")],
        [col("parent")],
        JoinArgs::new(JoinType::Left),
    );
    // replace NaN children with empty list
    final_communities["children"] = final_communities["children"].apply(
        |x| x if isinstance(x, np.ndarray) else []
    );
    // add fields for incremental update tracking
    final_communities["period"] = datetime.now(timezone.utc).date().isoformat();
    final_communities["size"] = final_communities.loc[:, "entity_ids"].apply(len);

    final_communities[..][
        COMMUNITIES_FINAL_COLUMNS,
    ]
}
