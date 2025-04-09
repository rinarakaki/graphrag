//! Content for the init CLI command to generate a default configuration.

use crate::config::defaults as defs;
use crate::config::defaults::{
    GRAPHRAG_CONFIG,
    LANGUAGE_MODEL,
    VECTOR_STORE,
  };

pub const INIT_YAML: String = format!(r#"\
### This config file contains required core defaults that must be set, along with a handful of common optional settings.
### For a full list of available settings, see https://microsoft.github.io/graphrag/config/yaml/

### LLM settings ###
## There are a number of settings to tune the threading and token limits for LLM calls - check the docs.

models:
  {defs.DEFAULT_CHAT_MODEL_ID}:
    type: {defs.DEFAULT_CHAT_MODEL_TYPE.value} # or azure_openai_chat
    # api_base: https://<instance>.openai.azure.com
    # api_version: 2024-05-01-preview
    auth_type: {defs.DEFAULT_CHAT_MODEL_AUTH_TYPE.value} # or azure_managed_identity
    api_key: ${{GRAPHRAG_API_KEY}} # set this in the generated .env file
    # audience: "https://cognitiveservices.azure.com/.default"
    # organization: <organization_id>
    model: {defs.DEFAULT_CHAT_MODEL}
    # deployment_name: <azure_model_deployment_name>
    # encoding_model: {defs.ENCODING_MODEL} # automatically set by tiktoken if left undefined
    model_supports_json: true # recommended if this is available for your model.
    concurrent_requests: {LANGUAGE_MODEL.concurrent_requests} # max number of simultaneous LLM requests allowed
    async_mode: {LANGUAGE_MODEL.async_mode.value} # or asyncio
    retry_strategy: native
    max_retries: -1                   # set to -1 for dynamic retry logic (most optimal setting based on server response)
    tokens_per_minute: 0              # set to 0 to disable rate limiting
    requests_per_minute: 0            # set to 0 to disable rate limiting
  {defs.DEFAULT_EMBEDDING_MODEL_ID}:
    type: {defs.DEFAULT_EMBEDDING_MODEL_TYPE.value} # or azure_openai_embedding
    # api_base: https://<instance>.openai.azure.com
    # api_version: 2024-05-01-preview
    auth_type: {defs.DEFAULT_EMBEDDING_MODEL_AUTH_TYPE.value} # or azure_managed_identity
    api_key: ${{GRAPHRAG_API_KEY}}
    # audience: "https://cognitiveservices.azure.com/.default"
    # organization: <organization_id>
    model: {defs.DEFAULT_EMBEDDING_MODEL}
    # deployment_name: <azure_model_deployment_name>
    # encoding_model: {defs.ENCODING_MODEL} # automatically set by tiktoken if left undefined
    model_supports_json: true # recommended if this is available for your model.
    concurrent_requests: {LANGUAGE_MODEL.concurrent_requests} # max number of simultaneous LLM requests allowed
    async_mode: {LANGUAGE_MODEL.async_mode.value} # or asyncio
    retry_strategy: native
    max_retries: -1                   # set to -1 for dynamic retry logic (most optimal setting based on server response)
    tokens_per_minute: 0              # set to 0 to disable rate limiting
    requests_per_minute: 0            # set to 0 to disable rate limiting

### Input settings ###

input:
  type: {GRAPHRAG_CONFIG.input.type.value} # or blob
  file_type: {GRAPHRAG_CONFIG.input.file_type.value} # [csv, text, json]
  base_dir: "{GRAPHRAG_CONFIG.input.base_dir}"

chunks:
  size: {GRAPHRAG_CONFIG.chunks.size}
  overlap: {GRAPHRAG_CONFIG.chunks.overlap}
  group_by_columns: [{",".join(GRAPHRAG_CONFIG.chunks.group_by_columns)}]

### Output/storage settings ###
## If blob storage is specified in the following four sections,
## connection_string and container_name must be provided

output:
  type: {GRAPHRAG_CONFIG.output.type.value} # [file, blob, cosmosdb]
  base_dir: "{GRAPHRAG_CONFIG.output.base_dir}"

cache:
  type: {GRAPHRAG_CONFIG.cache.type.value} # [file, blob, cosmosdb]
  base_dir: "{GRAPHRAG_CONFIG.cache.base_dir}"

reporting:
  type: {GRAPHRAG_CONFIG.reporting.type.value} # [file, blob, cosmosdb]
  base_dir: "{GRAPHRAG_CONFIG.reporting.base_dir}"

vector_store:
  {defs.DEFAULT_VECTOR_STORE_ID}:
    type: {VECTOR_STORE.type}
    db_uri: {VECTOR_STORE.db_uri}
    container_name: {VECTOR_STORE.container_name}
    overwrite: {VECTOR_STORE.overwrite}

### Workflow settings ###

embed_text:
  model_id: {GRAPHRAG_CONFIG.embed_text.model_id}
  vector_store_id: {GRAPHRAG_CONFIG.embed_text.vector_store_id}

extract_graph:
  model_id: {GRAPHRAG_CONFIG.extract_graph.model_id}
  prompt: "prompts/extract_graph.txt"
  entity_types: [{",".join(GRAPHRAG_CONFIG.extract_graph.entity_types)}]
  max_gleanings: {GRAPHRAG_CONFIG.extract_graph.max_gleanings}

summarize_descriptions:
  model_id: {GRAPHRAG_CONFIG.summarize_descriptions.model_id}
  prompt: "prompts/summarize_descriptions.txt"
  max_length: {GRAPHRAG_CONFIG.summarize_descriptions.max_length}

extract_graph_nlp:
  text_analyzer:
    extractor_type: {GRAPHRAG_CONFIG.extract_graph_nlp.text_analyzer.extractor_type.value} # [regex_english, syntactic_parser, cfg]

cluster_graph:
  max_cluster_size: {GRAPHRAG_CONFIG.cluster_graph.max_cluster_size}

extract_claims:
  enabled: false
  model_id: {GRAPHRAG_CONFIG.extract_claims.model_id}
  prompt: "prompts/extract_claims.txt"
  description: "{GRAPHRAG_CONFIG.extract_claims.description}"
  max_gleanings: {GRAPHRAG_CONFIG.extract_claims.max_gleanings}

community_reports:
  model_id: {GRAPHRAG_CONFIG.community_reports.model_id}
  graph_prompt: "prompts/community_report_graph.txt"
  text_prompt: "prompts/community_report_text.txt"
  max_length: {GRAPHRAG_CONFIG.community_reports.max_length}
  max_input_length: {GRAPHRAG_CONFIG.community_reports.max_input_length}

embed_graph:
  enabled: false # if true, will generate node2vec embeddings for nodes

umap:
  enabled: false # if true, will generate UMAP embeddings for nodes (embed_graph must also be enabled)

snapshots:
  graphml: false
  embeddings: false

### Query settings ###
## The prompt locations are required here, but each search method has a number of optional knobs that can be tuned.
## See the config docs: https://microsoft.github.io/graphrag/config/yaml/#query

local_search:
  chat_model_id: {GRAPHRAG_CONFIG.local_search.chat_model_id}
  embedding_model_id: {GRAPHRAG_CONFIG.local_search.embedding_model_id}
  prompt: "prompts/local_search_system_prompt.txt"

global_search:
  chat_model_id: {GRAPHRAG_CONFIG.global_search.chat_model_id}
  map_prompt: "prompts/global_search_map_system_prompt.txt"
  reduce_prompt: "prompts/global_search_reduce_system_prompt.txt"
  knowledge_prompt: "prompts/global_search_knowledge_system_prompt.txt"

drift_search:
  chat_model_id: {GRAPHRAG_CONFIG.drift_search.chat_model_id}
  embedding_model_id: {GRAPHRAG_CONFIG.drift_search.embedding_model_id}
  prompt: "prompts/drift_search_system_prompt.txt"
  reduce_prompt: "prompts/drift_search_reduce_prompt.txt"

basic_search:
  chat_model_id: {GRAPHRAG_CONFIG.basic_search.chat_model_id}
  embedding_model_id: {GRAPHRAG_CONFIG.basic_search.embedding_model_id}
  prompt: "prompts/basic_search_system_prompt.txt"
"#);

pub const INIT_DOTENV: &str = r#"\
GRAPHRAG_API_KEY=<API_KEY>
"#;
