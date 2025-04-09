//! CLI entrypoint.

// INVALID_METHOD_ERROR = "Invalid method"

// app = typer.Typer(
//     help="GraphRAG: A graph-based retrieval-augmented generation (RAG) system.",
//     no_args_is_help=True,
// )

// // A workaround for typer's lack of support for proper autocompletion of file/directory paths
// // For more detail, watch
// //   https://github.com/fastapi/typer/discussions/682
// //   https://github.com/fastapi/typer/issues/951
// pub fn path_autocomplete(
//     file_okay: bool = True,
//     dir_okay: bool = True,
//     readable: bool = True,
//     writable: bool = False,
//     match_wildcard: Option<String> /* = None */,
// ) -> Callable[[str], Vec<str>]:
//     /// Autocomplete file and directory paths.

//     def wildcard_match(string: str, pattern: str) -> bool:
//         regex = re.escape(pattern).replace(r"\?", ".").replace(r"\*", ".*")
//         return re.fullmatch(regex, string) is not None

//     use std::path::Path;

//     def completer(incomplete: str) -> Vec<str>:
//         # List items in the current directory as Path objects
//         items = Path().iterdir()
//         completions = []

//         for item in items:
//             # Filter based on file/directory properties
//             if not file_okay and item.is_file():
//                 continue
//             if not dir_okay and item.is_dir():
//                 continue
//             if readable and not os.access(item, os.R_OK):
//                 continue
//             if writable and not os.access(item, os.W_OK):
//                 continue

//             # push the name of the matching item
//             completions.push(item.name)

//         # Apply wildcard matching if required
//         if match_wildcard:
//             completions = filter(
//                 lambda i: wildcard_match(i, match_wildcard)
//                 if match_wildcard
//                 else False,
//                 completions,
//             )

//         # Return completions that start with the given incomplete string
//         return [i for i in completions if i.startswith(incomplete)]

//     return completer

// @app.command("init")
// def _initialize_cli(
//     root: Annotated[
//         Path,
//         typer.Option(
//             help="The project root directory.",
//             dir_okay=True,
//             writable=True,
//             resolve_path=True,
//             autocompletion=path_autocomplete(
//                 file_okay=False, dir_okay=True, writable=True, match_wildcard="*"
//             ),
//         ),
//     ],
//     force: Annotated[
//         bool,
//         typer.Option(help="Force initialization even if the project already exists."),
//     ] = False,
// ):
//     /// Generate a default configuration file.
//     use crate::cli.initialize::initialize_project_at

//     initialize_project_at(path=root, force=force)

// @app.command("index")
// def _index_cli(
//     config: Annotated[
//         Path | None,
//         typer.Option(
//             help="The configuration to use.",
//              exists=True,
//              file_okay=True, readable=True
//         ),
//     ] = None,
//     root: Annotated[
//         Path,
//         typer.Option(
//             help="The project root directory.",
//             exists=True,
//             dir_okay=True,
//             writable=True,
//             resolve_path=True,
//             autocompletion=path_autocomplete(
//                 file_okay=False, dir_okay=True, writable=True, match_wildcard="*"
//             ),
//         ),
//     ] = Path(),  # set default to current directory
//     method: Annotated[
//         IndexingMethod,
//         typer.Option(
//             help="The indexing method to use."
//         )
//     ] = IndexingMethod.Standard,
//     verbose: Annotated[
//         bool,
//         typer.Option(
//             help="Run the indexing pipeline with verbose logging"
//         )
//     ] = False,
//     memprofile: Annotated[
//         bool,
//         typer.Option(
//             help="Run the indexing pipeline with memory profiling"
//         )
//     ] = False,
//     logger: Annotated[
//         LoggerType,
//         typer.Option(
//             help="The progress logger to use.")
//     ] = LoggerType::Rich,
//     dry_run: Annotated[
//         bool,
//         typer.Option(
//             help="Run the indexing pipeline without executing any steps to inspect and validate the configuration."
//         ),
//     ] = False,
//     cache: Annotated[
//         bool,
//         typer.Option(help="Use LLM cache.")
//     ] = True,
//     skip_validation: Annotated[
//         bool,
//         typer.Option(
//             help="Skip any preflight validation. Useful when running no LLM steps."
//         ),
//     ] = False,
//     output: Annotated[
//         Path | None,
//         typer.Option(
//             help="Indexing pipeline output directory. Overrides output.base_dir in the configuration file.",
//             dir_okay=True,
//             writable=True,
//             resolve_path=True,
//         ),
//     ] = None,
// ):
//     /// Build a knowledge graph index.
//     use crate::cli.index::index_cli

//     index_cli(
//         root_dir=root,
//         verbose=verbose,
//         memprofile=memprofile,
//         cache=cache,
//         logger=LoggerType(logger),
//         config_filepath=config,
//         dry_run=dry_run,
//         skip_validation=skip_validation,
//         output_dir=output,
//         method=method,
//     )

// @app.command("update")
// def _update_cli(
//     config: Annotated[
//         Path | None,
//         typer.Option(
//             help="The configuration to use.", exists=True, file_okay=True, readable=True
//         ),
//     ] = None,
//     root: Annotated[
//         Path,
//         typer.Option(
//             help="The project root directory.",
//             exists=True,
//             dir_okay=True,
//             writable=True,
//             resolve_path=True,
//         ),
//     ] = Path(),  # set default to current directory
//     method: Annotated[
//         IndexingMethod, typer.Option(help="The indexing method to use.")
//     ] = IndexingMethod.Standard,
//     verbose: Annotated[
//         bool, typer.Option(help="Run the indexing pipeline with verbose logging")
//     ] = False,
//     memprofile: Annotated[
//         bool, typer.Option(help="Run the indexing pipeline with memory profiling")
//     ] = False,
//     logger: Annotated[
//         LoggerType, typer.Option(help="The progress logger to use.")
//     ] = LoggerType::Rich,
//     cache: Annotated[bool, typer.Option(help="Use LLM cache.")] = True,
//     skip_validation: Annotated[
//         bool,
//         typer.Option(
//             help="Skip any preflight validation. Useful when running no LLM steps."
//         ),
//     ] = False,
//     output: Annotated[
//         Path | None,
//         typer.Option(
//             help="Indexing pipeline output directory. Overrides output.base_dir in the configuration file.",
//             dir_okay=True,
//             writable=True,
//             resolve_path=True,
//         ),
//     ] = None,
// ):
//     """
//     Update an existing knowledge graph index.

//     Applies a default output configuration (if not provided by config), saving the new index to the local file system in the `update_output` folder.
//     """
//     use crate::cli.index::update_cli

//     update_cli(
//         root_dir=root,
//         verbose=verbose,
//         memprofile=memprofile,
//         cache=cache,
//         logger=LoggerType(logger),
//         config_filepath=config,
//         skip_validation=skip_validation,
//         output_dir=output,
//         method=method,
//     )

// @app.command("prompt-tune")
// def _prompt_tune_cli(
//     root: Annotated[
//         Path,
//         typer.Option(
//             help="The project root directory.",
//             exists=True,
//             dir_okay=True,
//             writable=True,
//             resolve_path=True,
//             autocompletion=path_autocomplete(
//                 file_okay=False, dir_okay=True, writable=True, match_wildcard="*"
//             ),
//         ),
//     ] = Path(),  # set default to current directory
//     config: Annotated[
//         Path | None,
//         typer.Option(
//             help="The configuration to use.",
//             exists=True,
//             file_okay=True,
//             readable=True,
//             autocompletion=path_autocomplete(
//                 file_okay=True, dir_okay=False, match_wildcard="*"
//             ),
//         ),
//     ] = None,
//     verbose: Annotated[
//         bool,
//         typer.Option(help="Run the prompt tuning pipeline with verbose logging")
//     ] = False,
//     logger: Annotated[
//         LoggerType, typer.Option(help="The progress logger to use.")
//     ] = LoggerType::Rich,
//     domain: Annotated[
//         Option<String>,
//         typer.Option(
//             help="The domain your input data is related to. For example 'space science', 'microbiology', 'environmental news'. If not defined, a domain will be inferred from the input data."
//         ),
//     ] = None,
//     selection_method: Annotated[
//         DocSelectionType, typer.Option(help="The text chunk selection method.")
//     ] = DocSelectionType.RANDOM,
//     n_subset_max: Annotated[
//         int,
//         typer.Option(
//             help="The number of text chunks to embed when --selection-method=auto."
//         ),
//     ] = N_SUBSET_MAX,
//     k: Annotated[
//         int,
//         typer.Option(
//             help="The maximum number of documents to select from each centroid when --selection-method=auto."
//         ),
//     ] = K,
//     limit: Annotated[
//         int,
//         typer.Option(
//             help="The number of documents to load when --selection-method={random,top}."
//         ),
//     ] = LIMIT,
//     max_tokens: Annotated[
//         int, typer.Option(help="The max token count for prompt generation.")
//     ] = MAX_TOKEN_COUNT,
//     min_examples_required: Annotated[
//         int,
//         typer.Option(
//             help="The minimum number of examples to generate/include in the entity extraction prompt."
//         ),
//     ] = 2,
//     chunk_size: Annotated[
//         int,
//         typer.Option(
//             help="The size of each example text chunk. Overrides chunks.size in the configuration file."
//         ),
//     ] = graphrag_config_defaults.chunks.size,
//     overlap: Annotated[
//         int,
//         typer.Option(
//             help="The overlap size for chunking documents. Overrides chunks.overlap in the configuration file"
//         ),
//     ] = graphrag_config_defaults.chunks.overlap,
//     language: Annotated[
//         Option<String>,
//         typer.Option(
//             help="The primary language used for inputs and outputs in graphrag prompts."
//         ),
//     ] = None,
//     discover_entity_types: Annotated[
//         bool, typer.Option(help="Discover and extract unspecified entity types.")
//     ] = True,
//     output: Annotated[
//         Path,
//         typer.Option(
//             help="The directory to save prompts to, relative to the project root directory.",
//             dir_okay=True,
//             writable=True,
//             resolve_path=True,
//         ),
//     ] = Path("prompts"),
// ):
//     /// Generate custom graphrag prompts with your own data (i.e. auto templating).
//    ::asyncio

//     use crate::cli.prompt_tune::prompt_tune

//     loop = asyncio.get_event_loop()
//     loop.run_until_complete(
//         prompt_tune(
//             root=root,
//             config=config,
//             domain=domain,
//             verbose=verbose,
//             logger=logger,
//             selection_method=selection_method,
//             limit=limit,
//             max_tokens=max_tokens,
//             chunk_size=chunk_size,
//             overlap=overlap,
//             language=language,
//             discover_entity_types=discover_entity_types,
//             output=output,
//             n_subset_max=n_subset_max,
//             k=k,
//             min_examples_required=min_examples_required,
//         )
//     )

// @app.command("query")
// def _query_cli(
//     method: Annotated[
//         SearchMethod,
//         typer.Option(help="The query algorithm to use.")],
//     query: Annotated[
//         str, typer.Option(help="The query to execute.")],
//     config: Annotated[
//         Path | None,
//         typer.Option(
//             help="The configuration to use.",
//             exists=True,
//             file_okay=True,
//             readable=True,
//             autocompletion=path_autocomplete(
//                 file_okay=True, dir_okay=False, match_wildcard="*"
//             ),
//         ),
//     ] = None,
//     data: Annotated[
//         Path | None,
//         typer.Option(
//             help="Indexing pipeline output directory (i.e. contains the parquet files).",
//             exists=True,
//             dir_okay=True,
//             readable=True,
//             resolve_path=True,
//             autocompletion=path_autocomplete(
//                 file_okay=False, dir_okay=True, match_wildcard="*"
//             ),
//         ),
//     ] = None,
//     root: Annotated[
//         Path,
//         typer.Option(
//             help="The project root directory.",
//             exists=True,
//             dir_okay=True,
//             writable=True,
//             resolve_path=True,
//             autocompletion=path_autocomplete(
//                 file_okay=False, dir_okay=True, match_wildcard="*"
//             ),
//         ),
//     ] = Path(),  # set default to current directory
//     community_level: Annotated[
//         int,
//         typer.Option(
//             help="The community level in the Leiden community hierarchy from which to load community reports. Higher values represent reports from smaller communities."
//         ),
//     ] = 2,
//     dynamic_community_selection: Annotated[
//         bool,
//         typer.Option(help="Use global search with dynamic community selection."),
//     ] = False,
//     response_type: Annotated[
//         str,
//         typer.Option(
//             help="Free form text describing the response type and format, can be anything, e.g. Multiple Paragraphs, Single Paragraph, Single Sentence, List of 3-7 Points, Single Page, Multi-Page Report. Default: Multiple Paragraphs"
//         ),
//     ] = "Multiple Paragraphs",
//     streaming: Annotated[
//         bool, typer.Option(help="Print response in a streaming manner.")
//     ] = False,
// ):
//     /// Query a knowledge graph index.
//     use crate::cli.query::(
//         run_basic_search,
//         run_drift_search,
//         run_global_search,
//         run_local_search,
//     )

//     match method:
//         case SearchMethod.LOCAL:
//             run_local_search(
//                 config_filepath=config,
//                 data_dir=data,
//                 root_dir=root,
//                 community_level=community_level,
//                 response_type=response_type,
//                 streaming=streaming,
//                 query=query,
//             )
//         case SearchMethod.GLOBAL:
//             run_global_search(
//                 config_filepath=config,
//                 data_dir=data,
//                 root_dir=root,
//                 community_level=community_level,
//                 dynamic_community_selection=dynamic_community_selection,
//                 response_type=response_type,
//                 streaming=streaming,
//                 query=query,
//             )
//         case SearchMethod.DRIFT:
//             run_drift_search(
//                 config_filepath=config,
//                 data_dir=data,
//                 root_dir=root,
//                 community_level=community_level,
//                 streaming=streaming,
//                 response_type=response_type,
//                 query=query,
//             )
//         case SearchMethod.BASIC:
//             run_basic_search(
//                 config_filepath=config,
//                 data_dir=data,
//                 root_dir=root,
//                 streaming=streaming,
//                 query=query,
//             )
//         case _:
//             raise ValueError(INVALID_METHOD_ERROR)

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use graphrag::config::defaults::GRAPHRAG_CONFIG;
use graphrag::config::enums::{IndexingMethod, SearchMethod};
use graphrag::logger::types::LoggerType;
use graphrag::prompt_tune::defaults::{K, LIMIT, MAX_TOKEN_COUNT, N_SUBSET_MAX};
use graphrag::prompt_tune::types::DocSelectionType;

use crate::index::index_cli;
use crate::index::update_cli;
use crate::initialize::initialize_project_at;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    kind: Kind,
}

#[derive(Debug, Subcommand)]
pub enum Kind {
    /// Generate a default configuration file.
    Init(Init),
    /// Build a knowledge graph index.
    Index(Index),
    /// Update an existing knowledge graph index.
    ///
    /// Applies a default output configuration (if not provided by config), saving the new index to the local file system in the `update_output` folder.
    Update(Update),
    /// Generate custom graphrag prompts with your own data (i.e. auto templating).
    PromptTune(PromptTune),
    /// Query a knowledge graph index.
    Query(Query),
}

#[derive(Debug, Parser)]
pub struct Init {
    /// The project root directory.
    #[arg(short, long)]
    root: PathBuf,
    /// Force initialization even if the project already exists.
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

#[derive(Debug, Parser)]
pub struct Index {
    /// The configuration to use.
    #[arg(short, long, default_value_t = None)]
    config: Option<Path>,
    /// The project root directory.
    #[arg(short, long)]
    root: Path,
    /// The indexing method to use.
    #[arg(short, long, default_value_t = IndexingMethod::Standard)]
    method: IndexingMethod,
    /// Run the indexing pipeline with verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    /// The progress logger to use.
    #[arg(short, long, default_value_t = LoggerType::Rich)]
    logger: LoggerType,
    /// Run the indexing pipeline without executing any steps to inspect and validate the configuration.
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
    /// Use LLM cache.
    #[arg(short, long, default_value_t = true)]
    cache: bool,
    /// Skip any preflight validation. Useful when running no LLM steps.
    #[arg(short, long, default_value_t = false)]
    skip_validation: bool,
    /// Indexing pipeline output directory. Overrides output.base_dir in the configuration file.
    #[arg(short, long, default_value_t = None)]
    output: Option<Path>,
}

#[derive(Debug, Parser)]
pub struct Update {
    /// The configuration to use.
    #[arg(short, long, default_value_t = None)]
    config: Option<Path>,
    /// The project root directory.
    #[arg(short, long)]
    root: Path,
    /// The indexing method to use.
    #[arg(short, long, default_value_t = IndexingMethod::Standard)]
    method: IndexingMethod,
    /// Run the indexing pipeline with verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    /// The progress logger to use.
    #[arg(short, long, default_value_t = LoggerType::Rich)]
    logger: LoggerType,
    /// Use LLM cache.
    #[arg(short, long, default_value_t = true)]
    cache: bool,
    /// Skip any preflight validation. Useful when running no LLM steps.
    #[arg(short, long, default_value_t = false)]
    skip_validation: bool,
    /// Indexing pipeline output directory. Overrides output.base_dir in the configuration file.
    #[arg(short, long, default_value_t = None)]
    output: Option<Path>,
}

#[derive(Debug, Parser)]
pub struct PromptTune {
    /// The project root directory.
    #[arg(short, long)]
    root: Path,
    /// The configuration to use.
    #[arg(short, long, default_value_t = None)]
    config: Option<Path>,
    /// Run the prompt tuning pipeline with verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    /// The progress logger to use.
    #[arg(short, long, default_value_t = LoggerType::Rich)]
    logger: LoggerType,
    /// The domain to map the input documents to.
    #[arg(short, long, default_value_t = None)]
    domain: Option<String>,
    /// The text chunk selection method.
    #[arg(short, long, default_value_t = DocSelectionType::Random)]
    selection_method: DocSelectionType,
    /// The number of text chunks to embed when --selection-method=auto.
    #[arg(short, long, default_value_t = N_SUBSET_MAX)]
    n_subset_max: usize,
    /// The maximum number of documents to select from each centroid when --selection-method=auto.
    #[arg(short, long, default_value_t = K)]
    k: usize,
    /// The number of documents to load when --selection-method={random,top}.
    #[arg(short, long, default_value_t = LIMIT)]
    limit: usize,
    /// The max token count for prompt generation.
    #[arg(short, long, default_value_t = MAX_TOKEN_COUNT)]
    max_tokens: usize,
    /// The minimum number of examples to generate/include in the entity extraction prompt.
    #[arg(short, long, default_value_t = 2)]
    min_examples_required: usize,
    /// The size of each example text chunk. Overrides chunks.size in the configuration file.
    #[arg(short, long, default_value_t = GRAPHRAG_CONFIG.chunks.size)]
    chunk_size: usize,
    /// The overlap size for chunking documents. Overrides chunks.overlap in the configuration file
    #[arg(short, long, default_value_t = GRAPHRAG_CONFIG.chunks.overlap)]
    overlap: usize,
    /// The primary language used for inputs and outputs in graphrag prompts.
    #[arg(short, long, default_value_t = None)]
    language: Option<String>,
    /// Discover and extract unspecified entity types.
    #[arg(short, long, default_value_t = true)]
    discover_entity_types: bool,
    /// The directory to save prompts to, relative to the project root directory.
    #[arg(short, long, default_value_t = "prompts")]
    output: Path,
}

#[derive(Debug, Parser)]
pub struct Query {
    /// The query algorithm to use.
    #[arg(short, long)]
    method: SearchMethod,
    /// The query to execute.
    #[arg(short, long)]
    query: String,
    /// The configuration to use.
    #[arg(short, long, default_value_t = None)]
    config: Option<Path>,
    /// Indexing pipeline output directory (i.e. contains the parquet files).
    #[arg(short, long)]
    data: Option<Path>,
    /// The project root directory.
    #[arg(short, long)]
    root: Path,
    /// The community level in the Leiden community hierarchy from which to load community reports. Higher values represent reports from smaller communities.
    #[arg(short, long, default_value_t = 2)]
    community_level: usize,
    /// Use global search with dynamic community selection.
    #[arg(short, long, default_value_t = false)]
    dynamic_community_selection: bool,
    /// Free form text describing the response type and format, can be anything, e.g. Multiple Paragraphs, Single Paragraph, Single Sentence, List of 3-7 Points, Single Page, Multi-Page Report. Default: Multiple Paragraphs
    #[arg(short, long, default_value_t = "Multiple Paragraphs")]
    response_type: String,
    /// Print response in a streaming manner.
    #[arg(short, long, default_value_t = false)]
    streaming: bool,
}

#[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.kind {
        Kind::Init(init) => initialize_project_at(path = root, force = force),
        Kind::Index(index) => index_cli(
            root_dir = index.root,
            verbose = index.verbose,
            memprofile = index.memprofile,
            cache = index.cache,
            logger = index.logger,
            config_filepath = index.config,
            dry_run = index.dry_run,
            skip_validation = index.skip_validation,
            output_dir = index.output,
            method = index.method,
        ),
        Kind::Update(update) => update_cli(
            root_dir = update.root,
            verbose = update.verbose,
            memprofile = update.memprofile,
            cache = update.cache,
            logger = update.logger,
            config_filepath = update.config,
            skip_validation = update.skip_validation,
            output_dir = update.output,
            method = update.method,
        ),
        Kind::PromptTune(prompt_tune) => prompt_tune(
            root = prompt_tune.root,
            config = prompt_tune.config,
            domain = prompt_tune.domain,
            verbose = prompt_tune.verbose,
            logger = prompt_tune.logger,
            selection_method = prompt_tune.selection_method,
            limit = prompt_tune.limit,
            max_tokens = prompt_tune.max_tokens,
            chunk_size = prompt_tune.chunk_size,
            overlap = prompt_tune.overlap,
            language = prompt_tune.language,
            discover_entity_types = prompt_tune.discover_entity_types,
            output = prompt_tune.output,
            n_subset_max = prompt_tune.n_subset_max,
            k = prompt_tune.k,
            min_examples_required = prompt_tune.min_examples_required,
        ),
        Kind::Query(query) => match query.method {
            SearchMethod::Local => run_local_search(
                config_filepath = query.config,
                data_dir = query.data,
                root_dir = query.root,
                community_level = query.community_level,
                response_type = query.response_type,
                streaming = query.streaming,
                query = query.query,
            ),
            SearchMethod::Global => run_global_search(
                config_filepath = query.config,
                data_dir = query.data,
                root_dir = query.root,
                community_level = query.community_level,
                dynamic_community_selection = query.dynamic_community_selection,
                response_type = query.response_type,
                streaming = query.streaming,
                query = query.query,
            ),
            SearchMethod::Drift => run_drift_search(
                config_filepath = query.config,
                data_dir = query.data,
                root_dir = query.root,
                community_level = query.community_level,
                streaming = query.streaming,
                response_type = query.response_type,
                query = query.query,
            ),
            SearchMethod::Basic => run_basic_search(
                config_filepath = query.config,
                data_dir = query.data,
                root_dir = query.root,
                streaming = query.streaming,
                query = query.query,
            ),
        },
    };
    Ok(())
}
