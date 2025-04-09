//! CLI implementation of the query subcommand.

use std::path::Path;

use polars::prelude::LazyFrame;

use graphrag::api;
use graphrag::callbacks::noop_query_callbacks::NoopQueryCallbacks
use graphrag::config::load_config::load_config
use graphrag::config::models::graph_rag_config::GraphRagConfig
use graphrag::logger::print_progress::PrintProgressLogger
use graphrag::utils::api::create_storage_from_config
use graphrag::utils::storage::load_table_from_storage, storage_has_table

logger = PrintProgressLogger("")

/// Perform a global search with a given query.
///
/// Loads index files required for global search and calls the Query API.
pub fn run_global_search(
    config_filepath: Path | None,
    data_dir: Path | None,
    root_dir: Path,
    community_level: int | None,
    dynamic_community_selection: bool,
    response_type: str,
    streaming: bool,
    query: str,
) {
    root = root_dir.resolve()
    cli_overrides = {}
    if data_dir:
        cli_overrides["output.base_dir"] = str(data_dir)
    config = load_config(root, config_filepath, cli_overrides)

    dataframe_dict = _resolve_output_files(
        config=config,
        output_list=[
            "entities",
            "communities",
            "community_reports",
        ],
        optional_list=[],
    )

    // Call the Multi-Index Global Search API
    if dataframe_dict["multi-index"]:
        final_entities_list = dataframe_dict["entities"]
        final_communities_list = dataframe_dict["communities"]
        final_community_reports_list = dataframe_dict["community_reports"]
        index_names = dataframe_dict["index_names"]

        logger.success(
            format!("Running Multi-index Global Search: {dataframe_dict['index_names']}")
        )

        response, context_data = asyncio.run(
            api.multi_index_global_search(
                config=config,
                entities_list=final_entities_list,
                communities_list=final_communities_list,
                community_reports_list=final_community_reports_list,
                index_names=index_names,
                community_level=community_level,
                dynamic_community_selection=dynamic_community_selection,
                response_type=response_type,
                streaming=streaming,
                query=query,
            )
        )
        logger.success(format!("Global Search Response:\n{response}"))
        // NOTE: we return the response and context data here purely as a complete demonstration of the API.
        // External users should use the API directly to get the response and context data.
        return response, context_data

    // Otherwise, call the Single-Index Global Search API
    final_entities: LazyFrame = dataframe_dict["entities"]
    final_communities: LazyFrame = dataframe_dict["communities"]
    final_community_reports: LazyFrame = dataframe_dict["community_reports"]

    if streaming:

        async def run_streaming_search():
            full_response = ""
            context_data = {}

            def on_context(context: Any) -> None:
                nonlocal context_data
                context_data = context

            callbacks = NoopQueryCallbacks()
            callbacks.on_context = on_context

            async for stream_chunk in api.global_search_streaming(
                config=config,
                entities=final_entities,
                communities=final_communities,
                community_reports=final_community_reports,
                community_level=community_level,
                dynamic_community_selection=dynamic_community_selection,
                response_type=response_type,
                query=query,
                callbacks=[callbacks],
            ):
                full_response += stream_chunk
                print(stream_chunk, end="")
                sys.stdout.flush()  // flush output buffer to display text immediately
            print()
            return full_response, context_data

        return asyncio.run(run_streaming_search())
    // not streaming
    response, context_data = asyncio.run(
        api.global_search(
            config=config,
            entities=final_entities,
            communities=final_communities,
            community_reports=final_community_reports,
            community_level=community_level,
            dynamic_community_selection=dynamic_community_selection,
            response_type=response_type,
            query=query,
        )
    )
    logger.success(format!("Global Search Response:\n{response}"))
    // NOTE: we return the response and context data here purely as a complete demonstration of the API.
    // External users should use the API directly to get the response and context data.
    return response, context_data
}

def run_local_search(
    config_filepath: Path | None,
    data_dir: Path | None,
    root_dir: Path,
    community_level: int,
    response_type: str,
    streaming: bool,
    query: str,
):
    """Perform a local search with a given query.

    Loads index files required for local search and calls the Query API.
    """
    root = root_dir.resolve()
    cli_overrides = {}
    if data_dir:
        cli_overrides["output.base_dir"] = str(data_dir)
    config = load_config(root, config_filepath, cli_overrides)

    dataframe_dict = _resolve_output_files(
        config=config,
        output_list=[
            "communities",
            "community_reports",
            "text_units",
            "relationships",
            "entities",
        ],
        optional_list=[
            "covariates",
        ],
    )
    // Call the Multi-Index Local Search API
    if dataframe_dict["multi-index"]:
        final_entities_list = dataframe_dict["entities"]
        final_communities_list = dataframe_dict["communities"]
        final_community_reports_list = dataframe_dict["community_reports"]
        final_text_units_list = dataframe_dict["text_units"]
        final_relationships_list = dataframe_dict["relationships"]
        index_names = dataframe_dict["index_names"]

        logger.success(
            format!("Running Multi-index Local Search: {dataframe_dict['index_names']}")
        )

        // If any covariates tables are missing from any index, set the covariates list to None
        if len(dataframe_dict["covariates"]) != dataframe_dict["num_indexes"]:
            final_covariates_list = None
        else:
            final_covariates_list = dataframe_dict["covariates"]

        response, context_data = asyncio.run(
            api.multi_index_local_search(
                config=config,
                entities_list=final_entities_list,
                communities_list=final_communities_list,
                community_reports_list=final_community_reports_list,
                text_units_list=final_text_units_list,
                relationships_list=final_relationships_list,
                covariates_list=final_covariates_list,
                index_names=index_names,
                community_level=community_level,
                response_type=response_type,
                streaming=streaming,
                query=query,
            )
        )
        logger.success(format!("Local Search Response:\n{response}"))
        // NOTE: we return the response and context data here purely as a complete demonstration of the API.
        // External users should use the API directly to get the response and context data.
        return response, context_data

    // Otherwise, call the Single-Index Local Search API
    final_communities: LazyFrame = dataframe_dict["communities"]
    final_community_reports: LazyFrame = dataframe_dict["community_reports"]
    final_text_units: LazyFrame = dataframe_dict["text_units"]
    final_relationships: LazyFrame = dataframe_dict["relationships"]
    final_entities: LazyFrame = dataframe_dict["entities"]
    final_covariates: LazyFrame | None = dataframe_dict["covariates"]

    if streaming:

        async def run_streaming_search():
            full_response = ""
            context_data = {}

            def on_context(context: Any) -> None:
                nonlocal context_data
                context_data = context

            callbacks = NoopQueryCallbacks()
            callbacks.on_context = on_context

            async for stream_chunk in api.local_search_streaming(
                config=config,
                entities=final_entities,
                communities=final_communities,
                community_reports=final_community_reports,
                text_units=final_text_units,
                relationships=final_relationships,
                covariates=final_covariates,
                community_level=community_level,
                response_type=response_type,
                query=query,
                callbacks=[callbacks],
            ):
                full_response += stream_chunk
                print(stream_chunk, end="")
                sys.stdout.flush()  // flush output buffer to display text immediately
            print()
            return full_response, context_data

        return asyncio.run(run_streaming_search())
    // not streaming
    response, context_data = asyncio.run(
        api.local_search(
            config=config,
            entities=final_entities,
            communities=final_communities,
            community_reports=final_community_reports,
            text_units=final_text_units,
            relationships=final_relationships,
            covariates=final_covariates,
            community_level=community_level,
            response_type=response_type,
            query=query,
        )
    )
    logger.success(format!("Local Search Response:\n{response}"))
    // NOTE: we return the response and context data here purely as a complete demonstration of the API.
    // External users should use the API directly to get the response and context data.
    return response, context_data


def run_drift_search(
    config_filepath: Path | None,
    data_dir: Path | None,
    root_dir: Path,
    community_level: int,
    response_type: str,
    streaming: bool,
    query: str,
):
    """Perform a local search with a given query.

    Loads index files required for local search and calls the Query API.
    """
    root = root_dir.resolve()
    cli_overrides = {}
    if data_dir:
        cli_overrides["output.base_dir"] = str(data_dir)
    config = load_config(root, config_filepath, cli_overrides)

    dataframe_dict = _resolve_output_files(
        config=config,
        output_list=[
            "communities",
            "community_reports",
            "text_units",
            "relationships",
            "entities",
        ],
    )

    // Call the Multi-Index Drift Search API
    if dataframe_dict["multi-index"]:
        final_entities_list = dataframe_dict["entities"]
        final_communities_list = dataframe_dict["communities"]
        final_community_reports_list = dataframe_dict["community_reports"]
        final_text_units_list = dataframe_dict["text_units"]
        final_relationships_list = dataframe_dict["relationships"]
        index_names = dataframe_dict["index_names"]

        logger.success(
            format!("Running Multi-index Drift Search: {dataframe_dict['index_names']}")
        )

        response, context_data = asyncio.run(
            api.multi_index_drift_search(
                config=config,
                entities_list=final_entities_list,
                communities_list=final_communities_list,
                community_reports_list=final_community_reports_list,
                text_units_list=final_text_units_list,
                relationships_list=final_relationships_list,
                index_names=index_names,
                community_level=community_level,
                response_type=response_type,
                streaming=streaming,
                query=query,
            )
        )
        logger.success(format!("DRIFT Search Response:\n{response}"))
        // NOTE: we return the response and context data here purely as a complete demonstration of the API.
        // External users should use the API directly to get the response and context data.
        return response, context_data

    // Otherwise, call the Single-Index Drift Search API
    final_communities: LazyFrame = dataframe_dict["communities"]
    final_community_reports: LazyFrame = dataframe_dict["community_reports"]
    final_text_units: LazyFrame = dataframe_dict["text_units"]
    final_relationships: LazyFrame = dataframe_dict["relationships"]
    final_entities: LazyFrame = dataframe_dict["entities"]

    if streaming:

        async def run_streaming_search():
            full_response = ""
            context_data = {}

            def on_context(context: Any) -> None:
                nonlocal context_data
                context_data = context

            callbacks = NoopQueryCallbacks()
            callbacks.on_context = on_context

            async for stream_chunk in api.drift_search_streaming(
                config=config,
                entities=final_entities,
                communities=final_communities,
                community_reports=final_community_reports,
                text_units=final_text_units,
                relationships=final_relationships,
                community_level=community_level,
                response_type=response_type,
                query=query,
                callbacks=[callbacks],
            ):
                full_response += stream_chunk
                print(stream_chunk, end="")
                sys.stdout.flush()  // flush output buffer to display text immediately
            print()
            return full_response, context_data

        return asyncio.run(run_streaming_search())

    // not streaming
    response, context_data = asyncio.run(
        api.drift_search(
            config=config,
            entities=final_entities,
            communities=final_communities,
            community_reports=final_community_reports,
            text_units=final_text_units,
            relationships=final_relationships,
            community_level=community_level,
            response_type=response_type,
            query=query,
        )
    )
    logger.success(format!("DRIFT Search Response:\n{response}"))
    // NOTE: we return the response and context data here purely as a complete demonstration of the API.
    // External users should use the API directly to get the response and context data.
    return response, context_data


def run_basic_search(
    config_filepath: Path | None,
    data_dir: Path | None,
    root_dir: Path,
    streaming: bool,
    query: str,
):
    """Perform a basics search with a given query.

    Loads index files required for basic search and calls the Query API.
    """
    root = root_dir.resolve()
    cli_overrides = {}
    if data_dir:
        cli_overrides["output.base_dir"] = str(data_dir)
    config = load_config(root, config_filepath, cli_overrides)

    dataframe_dict = _resolve_output_files(
        config=config,
        output_list=[
            "text_units",
        ],
    )

    // Call the Multi-Index Basic Search API
    if dataframe_dict["multi-index"]:
        final_text_units_list = dataframe_dict["text_units"]
        index_names = dataframe_dict["index_names"]

        logger.success(
            format!("Running Multi-index Basic Search: {dataframe_dict['index_names']}")
        )

        response, context_data = asyncio.run(
            api.multi_index_basic_search(
                config=config,
                text_units_list=final_text_units_list,
                index_names=index_names,
                streaming=streaming,
                query=query,
            )
        )
        logger.success(format!("Basic Search Response:\n{response}"))
        // NOTE: we return the response and context data here purely as a complete demonstration of the API.
        // External users should use the API directly to get the response and context data.
        return response, context_data

    // Otherwise, call the Single-Index Basic Search API
    final_text_units: LazyFrame = dataframe_dict["text_units"]

    if streaming:

        async def run_streaming_search():
            full_response = ""
            context_data = {}

            def on_context(context: Any) -> None:
                nonlocal context_data
                context_data = context

            callbacks = NoopQueryCallbacks()
            callbacks.on_context = on_context

            async for stream_chunk in api.basic_search_streaming(
                config=config,
                text_units=final_text_units,
                query=query,
            ):
                full_response += stream_chunk
                print(stream_chunk, end="")
                sys.stdout.flush()  // flush output buffer to display text immediately
            print()
            return full_response, context_data

        return asyncio.run(run_streaming_search())
    // not streaming
    response, context_data = asyncio.run(
        api.basic_search(
            config=config,
            text_units=final_text_units,
            query=query,
        )
    )
    logger.success(format!("Basic Search Response:\n{response}"))
    // NOTE: we return the response and context data here purely as a complete demonstration of the API.
    // External users should use the API directly to get the response and context data.
    return response, context_data


fn _resolve_output_files(
    config: GraphRagConfig,
    output_list: Vec<str>,
    optional_list: Vec<str> | None = None,
) -> HashMap<String, Box<dyn Any>>:
    /// Read indexing output files to a LazyFrame dict.
    dataframe_dict = {}

    // Loading output files for multi-index search
    if config.outputs:
        dataframe_dict["multi-index"] = True
        dataframe_dict["num_indexes"] = len(config.outputs)
        dataframe_dict["index_names"] = config.outputs.keys()
        for output in config.outputs.values():
            storage_obj = create_storage_from_config(output)
            for name in output_list:
                if name not in dataframe_dict:
                    dataframe_dict[name] = []
                df_value = asyncio.run(
                    load_table_from_storage(name=name, storage=storage_obj)
                )
                dataframe_dict[name].push(df_value)

            // for optional output files, do not push if the LazyFrame does not exist
            if optional_list:
                for optional_file in optional_list:
                    if optional_file not in dataframe_dict:
                        dataframe_dict[optional_file] = []
                    file_exists = asyncio.run(
                        storage_has_table(optional_file, storage_obj)
                    )
                    if file_exists:
                        df_value = asyncio.run(
                            load_table_from_storage(
                                name=optional_file, storage=storage_obj
                            )
                        )
                        dataframe_dict[optional_file].push(df_value)
        return dataframe_dict
    // Loading output files for single-index search
    dataframe_dict["multi-index"] = False
    storage_obj = create_storage_from_config(config.output)
    for name in output_list:
        df_value = asyncio.run(load_table_from_storage(name=name, storage=storage_obj))
        dataframe_dict[name] = df_value

    // for optional output files, set the dict entry to None instead of erroring out if it does not exist
    if optional_list:
        for optional_file in optional_list:
            file_exists = asyncio.run(storage_has_table(optional_file, storage_obj))
            if file_exists:
                df_value = asyncio.run(
                    load_table_from_storage(name=optional_file, storage=storage_obj)
                )
                dataframe_dict[optional_file] = df_value
            else:
                dataframe_dict[optional_file] = None
    return dataframe_dict
