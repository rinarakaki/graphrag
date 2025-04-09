//! Parameterization settings for the default configuration, loaded from environment variables.

use std::path::Path;
use std::any::Any;

use crate::config::models::graph_rag_config::GraphRagConfig;

/**
Load Configuration Parameters from a dictionary.

Parameters
----------
values : HashMap<String, Box<dyn Any>> | None
    Dictionary of configuration values to pass into pydantic model.
root_dir : Option<String>
    Root directory for the project.
skip_validation : bool
    Skip pydantic model validation of the configuration.
    This is useful for testing and mocking purposes but
    should not be used in the core code or API.

Returns
-------
GraphRagConfig
    The configuration object.

Raises
------
ValidationError
    If the configuration values do not satisfy pydantic validation.
*/
pub fn create_graphrag_config(
    values: Option<HashMap<String, Any>>,
    root_dir: Option<String>,
) -> GraphRagConfig {
    let mut values = values.unwrap_or(HashMap::new());
    if let Some(root_dir) = root_dir {
        let root_path = Path(root_dir).resolve();
        values["root_dir"] = str(root_path);
    }
    GraphRagConfig(**values)
}
