//! CLI functions for the GraphRAG module.

// import argparse
// import json
use std::path::Path;

def file_exist(path):
    /// Check for file existence.
    if not Path(path).is_file():
        msg = format!("File not found: {path}")
        raise argparse.ArgumentTypeError(msg)
    return path


def dir_exist(path):
    /// Check for directory existence.
    if not Path(path).is_dir():
        msg = format!("Directory not found: {path}")
        raise argparse.ArgumentTypeError(msg)
    return path


def redact(config: dict) -> str:
    /// Sanitize secrets in a config object.

    # Redact any sensitive configuration
    def redact_dict(config: dict) -> dict:
        if not isinstance(config, dict):
            return config

        result = {}
        for key, value in config.items():
            if key in {
                "api_key",
                "connection_string",
                "container_name",
                "organization",
            }:
                if value is not None:
                    result[key] = "==== REDACTED ===="
            elif isinstance(value, dict):
                result[key] = redact_dict(value)
            elif isinstance(value, list):
                result[key] = [redact_dict(i) for i in value]
            else:
                result[key] = value
        return result

    redacted_dict = redact_dict(config)
    return json.dumps(redacted_dict, indent=4)
