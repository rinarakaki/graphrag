//! CLI implementation of the initialization subcommand.

use std::path::Path;

use graphrag::config::init_content::{INIT_DOTENV, INIT_YAML};
use graphrag::logger::factory::{LoggerFactory, LoggerType};
use graphrag::prompts::index::community_report::COMMUNITY_REPORT_PROMPT;
use graphrag::prompts::index::community_report_text_units::COMMUNITY_REPORT_TEXT_PROMPT;
use graphrag::prompts::index::extract_claims::EXTRACT_CLAIMS_PROMPT;
use graphrag::prompts::index::extract_graph::GRAPH_EXTRACTION_PROMPT;
use graphrag::prompts::index::summarize_descriptions::SUMMARIZE_PROMPT;
use graphrag::prompts::query::basic_search_system_prompt::BASIC_SEARCH_SYSTEM_PROMPT;
use graphrag::prompts::query::drift_search_system_prompt::{
    DRIFT_LOCAL_SYSTEM_PROMPT,
    DRIFT_REDUCE_PROMPT,
};
use graphrag::prompts::query::global_search_knowledge_system_prompt::GENERAL_KNOWLEDGE_INSTRUCTION;
use graphrag::prompts::query::global_search_map_system_prompt::MAP_SYSTEM_PROMPT;
use graphrag::prompts::query::global_search_reduce_system_prompt::REDUCE_SYSTEM_PROMPT;
use graphrag::prompts::query::local_search_system_prompt::LOCAL_SEARCH_SYSTEM_PROMPT;
use graphrag::prompts::query::question_gen_system_prompt::QUESTION_SYSTEM_PROMPT;

/**
Initialize the project at the given path.

Parameters
----------
path : Path
    The path at which to initialize the project.
force : bool
    Whether to force initialization even if the project already exists.

Raises
------
ValueError
    If the project already exists and force is False.
 */
pub fn initialize_project_at(path: Path, force: bool) {
    let progress_logger = LoggerFactory().create_logger(LoggerType::Rich);
    progress_logger.info(format!("Initializing project at {path}"));
    let root = Path(path);
    if not root.exists() {
        root.mkdir(parents=True, exist_ok=True)
    }

    let settings_yaml = root / "settings.yaml";
    if settings_yaml.exists() and not force {
        msg = format!("Project already initialized at {root}")
        raise ValueError(msg)
    }

    {
        with settings_yaml.open("wb") as file:
        file.write(INIT_YAML.encode(encoding="utf-8", errors="strict"))
    }

    let dotenv = root / ".env";
    if not dotenv.exists() or force {
        with dotenv.open("wb") as file:
            file.write(INIT_DOTENV.encode(encoding="utf-8", errors="strict"))
    }

    let prompts_dir = root / "prompts";
    if not prompts_dir.exists() {
        prompts_dir.mkdir(parents=True, exist_ok=True);
    }

    let prompts = HashMap::from([
        ("extract_graph", GRAPH_EXTRACTION_PROMPT),
        ("summarize_descriptions", SUMMARIZE_PROMPT),
        ("extract_claims", EXTRACT_CLAIMS_PROMPT),
        ("community_report_graph", COMMUNITY_REPORT_PROMPT),
        ("community_report_text", COMMUNITY_REPORT_TEXT_PROMPT),
        ("drift_search_system_prompt", DRIFT_LOCAL_SYSTEM_PROMPT),
        ("drift_reduce_prompt", DRIFT_REDUCE_PROMPT),
        ("global_search_map_system_prompt", MAP_SYSTEM_PROMPT),
        ("global_search_reduce_system_prompt", REDUCE_SYSTEM_PROMPT),
        ("global_search_knowledge_system_prompt", GENERAL_KNOWLEDGE_INSTRUCTION),
        ("local_search_system_prompt", LOCAL_SEARCH_SYSTEM_PROMPT),
        ("basic_search_system_prompt", BASIC_SEARCH_SYSTEM_PROMPT),
        ("question_gen_system_prompt", QUESTION_SYSTEM_PROMPT),
    ]);

    for (name, content) in prompts.items() {
        let prompt_file = prompts_dir / format!("{name}.txt");
        if not prompt_file.exists() or force {
            with prompt_file.open("wb") as file:
                file.write(content.encode(encoding="utf-8", errors="strict"))
        }
    }
}
