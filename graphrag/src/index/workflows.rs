//! A package containing all built-in workflow definitions.

pub mod create_base_text_units;
pub mod create_communities;
pub mod extract_graph;
pub mod generate_text_embeddings;

// use crate::index.workflows.factory::PipelineFactory

// from .create_base_text_units::(
//     run_workflow as run_create_base_text_units,
// )
// from .create_communities::(
//     run_workflow as run_create_communities,
// )
// from .create_community_reports::(
//     run_workflow as run_create_community_reports,
// )
// from .create_community_reports_text::(
//     run_workflow as run_create_community_reports_text,
// )
// from .create_final_documents::(
//     run_workflow as run_create_final_documents,
// )
// from .create_final_text_units::(
//     run_workflow as run_create_final_text_units,
// )
// from .extract_covariates::(
//     run_workflow as run_extract_covariates,
// )
// from .extract_graph::(
//     run_workflow as run_extract_graph,
// )
// from .extract_graph_nlp::(
//     run_workflow as run_extract_graph_nlp,
// )
// from .finalize_graph::(
//     run_workflow as run_finalize_graph,
// )
// from .generate_text_embeddings::(
//     run_workflow as run_generate_text_embeddings,
// )
// from .prune_graph::(
//     run_workflow as run_prune_graph,
// )

// # register all of our built-in workflows at once
// PipelineFactory.register_all({
//     "create_base_text_units": run_create_base_text_units,
//     "create_communities": run_create_communities,
//     "create_community_reports_text": run_create_community_reports_text,
//     "create_community_reports": run_create_community_reports,
//     "extract_covariates": run_extract_covariates,
//     "create_final_documents": run_create_final_documents,
//     "create_final_text_units": run_create_final_text_units,
//     "extract_graph_nlp": run_extract_graph_nlp,
//     "extract_graph": run_extract_graph,
//     "finalize_graph": run_finalize_graph,
//     "generate_text_embeddings": run_generate_text_embeddings,
//     "prune_graph": run_prune_graph,
// })
