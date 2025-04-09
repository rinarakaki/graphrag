//! CLI implementation of the index subcommand.

use log::{info, error};
use std::path::Path;

use graphrag::api;
use graphrag::config::enums::{CacheType, IndexingMethod};
use graphrag::config::load_config::load_config;
use graphrag::config::logging::enable_logging_with_config;
use graphrag::index::validate_config::validate_config_names;
use graphrag::logger::base::ProgressLogger;
use graphrag::logger::factory::{LoggerFactory, LoggerType};
use graphrag::utils::cli::redact;

fn _logger(logger: ProgressLogger) {
    fn info(msg: &str, verbose: bool /* = false */) {
        info!(msg);
        if verbose{
            logger.info(msg)}
    }

    fn error(msg: &str, verbose: bool /* = false */) {
        error!(msg);
        if verbose{
            logger.error(msg)}
    }

    fn success(msg: &str, verbose: bool /* = false */) {
        info!(msg);
        if verbose{
            logger.success(msg)}
    }

    (info, error, success)
}

fn _register_signal_handlers(logger: ProgressLogger) {
//    ::signal

    fn handle_signal(signum, _) {
        // Handle the signal here
        logger.info(format!("Received signal {signum}, exiting..."));
        logger.dispose();
        for task in asyncio.all_tasks() {
            task.cancel();
        }
        logger.info("All tasks cancelled. Exiting...");
    }

    // Register signal handlers for SIGINT and SIGHUP
    signal.signal(signal.SIGINT, handle_signal);

    if sys.platform != "win32" {
        signal.signal(signal.SIGHUP, handle_signal)
    }
}

/// Run the pipeline with the given config.
pub fn index_cli(
    root_dir: Path,
    method: IndexingMethod,
    verbose: bool,
    memprofile: bool,
    cache: bool,
    logger: LoggerType,
    config_filepath: Option<Path>,
    dry_run: bool,
    skip_validation: bool,
    output_dir: Option<Path>,
) {
    let cli_overrides = {};
    if output_dir {
        cli_overrides["output.base_dir"] = str(output_dir);
        cli_overrides["reporting.base_dir"] = str(output_dir);
        cli_overrides["update_index_output.base_dir"] = str(output_dir);
    }
    let config = load_config(root_dir, config_filepath, cli_overrides);

    _run_index(
        config=config,
        method=method,
        is_update_run=False,
        verbose=verbose,
        memprofile=memprofile,
        cache=cache,
        logger=logger,
        dry_run=dry_run,
        skip_validation=skip_validation,
    )
}

/// Run the pipeline with the given config.
pub fn update_cli(
    root_dir: Path,
    method: IndexingMethod,
    verbose: bool,
    memprofile: bool,
    cache: bool,
    logger: LoggerType,
    config_filepath: Option<Path>,
    skip_validation: bool,
    output_dir: Option<Path>,
) {
    let cli_overrides = {};
    if output_dir {
        cli_overrides["output.base_dir"] = str(output_dir);
        cli_overrides["reporting.base_dir"] = str(output_dir);
        cli_overrides["update_index_output.base_dir"] = str(output_dir);
    }

    let config = load_config(root_dir, config_filepath, cli_overrides);

    _run_index(
        config=config,
        method=method,
        is_update_run=True,
        verbose=verbose,
        memprofile=memprofile,
        cache=cache,
        logger=logger,
        dry_run=False,
        skip_validation=skip_validation,
    )
}

fn _run_index(
    config: GraphRagConfig,
    method: IndexingMethod,
    is_update_run: bool,
    verbose: bool,
    memprofile: bool,
    cache: bool,
    logger: LoggerType,
    dry_run: bool,
    skip_validation: bool,
) {
    let progress_logger = LoggerFactory().create_logger(logger);
    let (info, error, success) = _logger(progress_logger);

    if !cache {
        config.cache.r#type = CacheType::None;
    }

    let (enabled_logging, log_path) = enable_logging_with_config(config, verbose);
    if enabled_logging {
        info("Logging enabled at {log_path}", True)
    }else {
        info(
            "Logging not enabled for config {redact(config.model_dump())}",
            True,
        )}

    if !skip_validation {
        validate_config_names(progress_logger, config)
    }

    info("Starting pipeline run. {dry_run=}", verbose);
    info(
        "Using default configuration: {redact(config.model_dump())}",
        verbose,
    );

    if dry_run {
        info("Dry run complete, exiting...", True);
        sys.exit(0);}

    _register_signal_handlers(progress_logger);

    let outputs = asyncio.run(
        api.build_index(
            config=config,
            method=method,
            is_update_run=is_update_run,
            memory_profile=memprofile,
            progress_logger=progress_logger,
        )
    );
    let encountered_errors = any(
        output.errors && len(output.errors) > 0 for output in outputs
    );

    let progress_logger.stop();
    if encountered_errors {
        error(
            "Errors occurred during the pipeline run, see logs for more details.", True
        );
    }else {
        success("All workflows completed successfully.", True);}

    sys::exit(1 if encountered_errors else 0)
}
