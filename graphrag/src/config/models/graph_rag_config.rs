//! Parameterization settings for the default configuration.

// from devtools::pformat

use std::collections::HashMap;

use crate::config::enums::OutputType;
// use crate::config::errors::LanguageModelConfigMissingError;
use crate::config::defaults::DEFAULT_VECTOR_STORE_ID;
use crate::config::models::basic_search_config::BasicSearchConfig;
use crate::config::models::cache_config::CacheConfig;
use crate::config::models::chunking_config::ChunkingConfig;
use crate::config::models::cluster_graph_config::ClusterGraphConfig;
use crate::config::models::community_reports_config::CommunityReportsConfig;
use crate::config::models::drift_search_config::DRIFTSearchConfig;
use crate::config::models::embed_graph_config::EmbedGraphConfig;
use crate::config::models::extract_claims_config::ClaimExtractionConfig;
use crate::config::models::extract_graph_config::ExtractGraphConfig;
use crate::config::models::extract_graph_nlp_config::ExtractGraphNLPConfig;
use crate::config::models::global_search_config::GlobalSearchConfig;
use crate::config::models::input_config::InputConfig;
use crate::config::models::language_model_config::LanguageModelConfig;
use crate::config::models::local_search_config::LocalSearchConfig;
use crate::config::models::output_config::OutputConfig;
use crate::config::models::prune_graph_config::PruneGraphConfig;
use crate::config::models::reporting_config::ReportingConfig;
use crate::config::models::snapshots_config::SnapshotsConfig;
use crate::config::models::summarize_descriptions_config::SummarizeDescriptionsConfig;
use crate::config::models::text_embedding_config::TextEmbeddingConfig;
use crate::config::models::umap_config::UmapConfig;
use crate::config::models::vector_store_config::VectorStoreConfig;
// use crate::vector_stores::factory::VectorStoreType;

/// Base class for the Default-Configuration parameterization settings.
pub struct GraphRagConfig {
    /// The root directory for the configuration.
    pub root_dir: String,

    /// Available language model configurations.
    pub models: HashMap<String, LanguageModelConfig>,

    /// The input configuration.
    pub input: InputConfig,

    /// The chunking configuration to use.
    pub chunks: ChunkingConfig,

    /// The output configuration.
    pub output: OutputConfig,

    /// A list of output configurations used for multi-index query.
    pub outputs: Option<HashMap<String, OutputConfig>>,

    /// The output configuration for the updated index.
    pub update_index_output: OutputConfig,

    /// The cache configuration.
    pub cache: CacheConfig,

    /// The reporting configuration.
    pub reporting: ReportingConfig,

    /// The vector store configuration.
    pub vector_store: HashMap<String, VectorStoreConfig>,

    /// List of workflows to run, in execution order. This always overrides any built-in workflow methods.
    pub workflows: Option<Vec<String>>,

    /// Text embedding configuration.
    pub embed_text: TextEmbeddingConfig,

    /// The entity extraction configuration to use.
    pub extract_graph: ExtractGraphConfig,

    /// The description summarization configuration to use.
    pub summarize_descriptions: SummarizeDescriptionsConfig,

    /// The NLP-based graph extraction configuration to use.
    pub extract_graph_nlp: ExtractGraphNLPConfig,

    /// The graph pruning configuration to use.
    pub prune_graph: PruneGraphConfig,

    /// The cluster graph configuration to use.
    pub cluster_graph: ClusterGraphConfig,

    /// The claim extraction configuration to use.
    pub extract_claims: ClaimExtractionConfig,

    /// The community reports configuration to use.
    pub community_reports: CommunityReportsConfig,

    /// Graph embedding configuration.
    pub embed_graph: EmbedGraphConfig,

    /// The UMAP configuration to use.
    pub umap: UmapConfig,

    /// The snapshots configuration to use.
    pub snapshots: SnapshotsConfig,

    /// The local search configuration.
    pub local_search: LocalSearchConfig,

    /// The global search configuration.
    pub global_search: GlobalSearchConfig,

    /// The drift search configuration.
    pub drift_search: DRIFTSearchConfig,

    /// The basic search configuration.
    pub basic_search: BasicSearchConfig,
}

impl Default for GraphRagConfig {
    /// Default values for GraphRAG.
    fn default() -> Self {
        GraphRagConfig {
            root_dir: "".into(),
            models: HashMap::new(),
            input: InputConfig::default(),
            chunks: ChunkingConfig::default(),
            output: OutputConfig::default(),
            outputs: None,
            update_index_output: OutputConfig {
                r#type: OutputType::File,
                base_dir: "update_output".into(),
                ..OutputConfig::default()
            },
            cache: CacheConfig::default(),
            reporting: ReportingConfig::default(),
            vector_store: HashMap::from([(
                DEFAULT_VECTOR_STORE_ID.into(),
                VectorStoreConfig::default(),
            )]),
            workflows: None,
            embed_text: TextEmbeddingConfig::default(),
            extract_graph: ExtractGraphConfig::default(),
            summarize_descriptions: SummarizeDescriptionsConfig::default(),
            extract_graph_nlp: ExtractGraphNLPConfig::default(),
            prune_graph: PruneGraphConfig::default(),
            cluster_graph: ClusterGraphConfig::default(),
            extract_claims: ClaimExtractionConfig::default(),
            community_reports: CommunityReportsConfig::default(),
            embed_graph: EmbedGraphConfig::default(),
            umap: UmapConfig::default(),
            snapshots: SnapshotsConfig::default(),
            local_search: LocalSearchConfig::default(),
            global_search: GlobalSearchConfig::default(),
            drift_search: DRIFTSearchConfig::default(),
            basic_search: BasicSearchConfig::default(),
        }
    }
}

impl GraphRagConfig {
    //     def __repr__(self) -> str:
    //         /// Get a string representation.
    //         return pformat(self, highlight=False)

    //     def __str__(self):
    //         /// Get a string representation.
    //         return self.model_dump_json(indent=4)

    //     def _validate_root_dir(self) -> None:
    //         /// Validate the root directory.
    //         if self.root_dir.strip() == "":
    //             self.root_dir = str(Path.cwd())

    //         root_dir = Path(self.root_dir).resolve()
    //         if not root_dir.is_dir():
    //             msg = format!("Invalid root directory: {self.root_dir} is not a directory.")
    //             raise FileNotFoundError(msg)
    //         self.root_dir = str(root_dir)

    //     def _validate_models(self) -> None:
    //         """Validate the models configuration.

    //         Ensure both a default chat model and default embedding model
    //         have been defined. Other models may also be defined but
    //         defaults are required for the time being as places of the
    //         code fallback to default model configs instead
    //         of specifying a specific model.

    //         TODO: Don't fallback to default models elsewhere in the code.
    //         Forcing code to specify a model to use and allowing for any
    //         names for model configurations.
    //         """
    //         if defs.DEFAULT_CHAT_MODEL_ID not in self.models:
    //             raise LanguageModelConfigMissingError(defs.DEFAULT_CHAT_MODEL_ID)
    //         if defs.DEFAULT_EMBEDDING_MODEL_ID not in self.models:
    //             raise LanguageModelConfigMissingError(defs.DEFAULT_EMBEDDING_MODEL_ID)

    //     def _validate_input_pattern(self) -> None:
    //         /// Validate the input file pattern based on the specified type.
    //         if len(self.input.file_pattern) == 0:
    //             if self.input.file_type == defs.InputFileType.text:
    //                 self.input.file_pattern = ".*\\.txt$"
    //             else:
    //                 self.input.file_pattern = format!(".*\\.{self.input.file_type.value}$")

    //     def _validate_output_base_dir(self) -> None:
    //         /// Validate the output base directory.
    //         if self.output.type == defs.OutputType.file:
    //             if self.output.base_dir.strip() == "":
    //                 msg = "output base directory is required for file output. Please rerun `graphrag init` and set the output configuration."
    //                 raise ValueError(msg)
    //             self.output.base_dir = str(
    //                 (Path(self.root_dir) / self.output.base_dir).resolve()
    //             )

    //     def _validate_multi_output_base_dirs(self) -> None:
    //         /// Validate the outputs dict base directories.
    //         if self.outputs:
    //             for output in self.outputs.values():
    //                 if output.type == defs.OutputType.file:
    //                     if output.base_dir.strip() == "":
    //                         msg = "Output base directory is required for file output. Please rerun `graphrag init` and set the output configuration."
    //                         raise ValueError(msg)
    //                     output.base_dir = str(
    //                         (Path(self.root_dir) / output.base_dir).resolve()
    //                     )

    //     def _validate_update_index_output_base_dir(self) -> None:
    //         /// Validate the update index output base directory.
    //         if self.update_index_output.type == defs.OutputType.file:
    //             if self.update_index_output.base_dir.strip() == "":
    //                 msg = "update_index_output base directory is required for file output. Please rerun `graphrag init` and set the update_index_output configuration."
    //                 raise ValueError(msg)
    //             self.update_index_output.base_dir = str(
    //                 (Path(self.root_dir) / self.update_index_output.base_dir).resolve()
    //             )

    //     def _validate_reporting_base_dir(self) -> None:
    //         /// Validate the reporting base directory.
    //         if self.reporting.type == defs.ReportingType.file:
    //             if self.reporting.base_dir.strip() == "":
    //                 msg = "Reporting base directory is required for file reporting. Please rerun `graphrag init` and set the reporting configuration."
    //                 raise ValueError(msg)
    //             self.reporting.base_dir = str(
    //                 (Path(self.root_dir) / self.reporting.base_dir).resolve()
    //             )

    //     def _validate_vector_store_db_uri(self) -> None:
    //         /// Validate the vector store configuration.
    //         for store in self.vector_store.values():
    //             if store.type == VectorStoreType.LanceDB:
    //                 if not store.db_uri or store.db_uri.strip == "":
    //                     msg = "Vector store URI is required for LanceDB. Please rerun `graphrag init` and set the vector store configuration."
    //                     raise ValueError(msg)
    //                 store.db_uri = str((Path(self.root_dir) / store.db_uri).resolve())

    /**
    Get a model configuration by ID.

    Parameters
    ----------
    model_id : str
        The ID of the model to get. Should match an ID in the models list.

    Returns
    -------
    LanguageModelConfig
        The model configuration if found.

    Raises
    ------
    ValueError
        If the model ID is not found in the configuration.
    */
    pub fn get_language_model_config(&self, model_id: &str) -> &LanguageModelConfig {
        assert!(
            self.models.contains_key(model_id),
            "ValueError: Model ID {model_id} not found in configuration. Please rerun `graphrag init` and set the model configuration.",
        );

        &self.models[model_id]
    }

    /**
    Get a vector store configuration by ID.

    Parameters
    ----------
    vector_store_id : str
        The ID of the vector store to get. Should match an ID in the vector_store list.

    Returns
    -------
    VectorStoreConfig
        The vector store configuration if found.

    Raises
    ------
    ValueError
        If the vector store ID is not found in the configuration.
    */
    pub fn get_vector_store_config(self, vector_store_id: &str) -> &VectorStoreConfig {
        assert!(
            self.vector_store.contains_key(vector_store_id),
            "Vector Store ID {vector_store_id} not found in configuration. Please rerun `graphrag init` and set the vector store configuration.",
        );

        &self.vector_store[vector_store_id]
    }

    //     @model_validator(mode="after")
    //     def _validate_model(self):
    //         /// Validate the model configuration.
    //         self._validate_root_dir()
    //         self._validate_models()
    //         self._validate_input_pattern()
    //         self._validate_reporting_base_dir()
    //         self._validate_output_base_dir()
    //         self._validate_multi_output_base_dirs()
    //         self._validate_update_index_output_base_dir()
    //         self._validate_vector_store_db_uri()
    //         return self
}
