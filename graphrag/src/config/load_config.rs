//! Default method for loading config.

use std::path::Path;
// from string::Template

// from dotenv::load_dotenv

use crate::config::create_graphrag_config::create_graphrag_config;
use crate::config::models::graph_rag_config::GraphRagConfig;

_default_config_files = ["settings.yaml", "settings.yml", "settings.json"]

/**
Resolve the config path from the given root directory.

Parameters
----------
root : str | Path
    The path to the root directory containing the config file.
    Searches for a default config file (settings.{yaml,yml,json}).

Returns
-------
Path | None
    returns a Path if there is a config in the root directory
    Otherwise returns None.
 */
fn _search_for_config_in_root_dir(root: str | Path) -> Option<Path> {
    let root = Path(root);

    if not root.is_dir() {
        msg = format!("Invalid config path: {root} is not a directory")
        raise FileNotFoundError(msg)
    }

    for file in _default_config_files {
        if (root / file).is_file() {
            return root / file
        }
    }

    None
}

/**
Parse environment variables in the configuration text.

Parameters
----------
text : str
    The configuration text.

Returns
-------
str
    The configuration text with environment variables parsed.

Raises
------
KeyError
    If an environment variable is not found.
 */
fn _parse_env_variables(text: str) -> str {
    Template(text).substitute(os.environ)
}

/**
Load the .env file if it exists in the same directory as the config file.

Parameters
----------
config_path : Path | str
    The path to the config file.
 */
fn _load_dotenv(config_path: Path | str) {
    let config_path = Path(config_path);
    let dotenv_path = config_path.parent / ".env";
    if dotenv_path.exists() {
        load_dotenv(dotenv_path);
    }
}

/**
Get the configuration file path.

Parameters
----------
root_dir : str | Path
    The root directory of the project. Will search for the config file in this directory.
config_filepath : Option<String>
    The path to the config file.
    If None, searches for config file in root.

Returns
-------
Path
    The configuration file path.
 */
fn _get_config_path(root_dir: Path, config_filepath: Path | None) -> Path {
    let config_path = if config_filepath {
        let config_path = config_filepath.resolve();
        if not config_path.exists() {
            msg = format!("Specified Config file not found: {config_path}")
            raise FileNotFoundError(msg)
        }
    } else {
        _search_for_config_in_root_dir(root_dir)
    };

    if not config_path {
        msg = format!("Config file not found in root directory: {root_dir}")
        raise FileNotFoundError(msg)
    }

    config_path
}

/// Apply the overrides to the raw configuration.
fn _apply_overrides(data: HashMap<String, Box<dyn Any>>, overrides: HashMap<String, Box<dyn Any>>) {
    for (key, value) in overrides.items() {
        let keys = key.split(".")
        let target = data;
        let mut current_path = keys[0]
        for k in keys[:-1] {
            current_path += format!(".{k}")
            target_obj = target.get(k, {})
            if not isinstance(target_obj, dict):
                msg = format!("Cannot override non-dict value: data[{current_path}] is not a dict.")
                raise TypeError(msg)
            target[k] = target_obj
            target = target[k]
        }
        target[keys[-1]] = value;
    }
}

/// Parse configuration.
fn _parse(file_extension: str, contents: str) -> HashMap<String, Box<dyn Any>> {
    match file_extension {
        case ".yaml" | ".yml":
            return yaml.safe_load(contents)
        case ".json":
            return json.loads(contents)
        case _:
            msg = (
                format!("Unable to parse config. Unsupported file extension: {file_extension}")
            )
            raise ValueError(msg)
    }
}

/**
Load configuration from a file.

Parameters
----------
root_dir : str | Path
    The root directory of the project. Will search for the config file in this directory.
config_filepath : Option<String>
    The path to the config file.
    If None, searches for config file in root.
cli_overrides : HashMap<String, Box<dyn Any>> | None
    A flat dictionary of cli overrides.
    Example: {'output.base_dir': 'override_value'}

Returns
-------
GraphRagConfig
    The loaded configuration.

Raises
------
FileNotFoundError
    If the config file is not found.
ValueError
    If the config file extension is not supported.
TypeError
    If applying cli overrides to the config fails.
KeyError
    If config file references a non-existent environment variable.
ValidationError
    If there are pydantic validation errors when instantiating the config.
 */
pub fn load_config(
    root_dir: Path,
    config_filepath: Path | None = None,
    cli_overrides: HashMap<String, Box<dyn Any>> | None = None,
) -> GraphRagConfig {
    let root = root_dir.resolve();
    let config_path = _get_config_path(root, config_filepath)
    _load_dotenv(config_path)
    let config_extension = config_path.suffix
    let config_text = config_path.read_text(encoding="utf-8")
    let config_text = _parse_env_variables(config_text)
    let config_data = _parse(config_extension, config_text)
    if cli_overrides {
        _apply_overrides(config_data, cli_overrides)
    }
    create_graphrag_config(config_data, root_dir=str(root))
}
