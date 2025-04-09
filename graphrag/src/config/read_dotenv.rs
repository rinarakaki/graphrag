//! A module containing the read_dotenv utility.

use std::path::Path;

use log;

// from dotenv::dotenv_values


/// Read a .env file in the given root path.
pub fn read_dotenv(root: str) {
    env_path = Path(root) / ".env"
    if env_path.exists() {
        info!("Loading pipeline .env file")
        let env_config = dotenv_values(format!("{env_path}"));
        for key, value in env_config.items() {
            if key not in os.environ:
                os.environ[key] = value or ""
        }
    } else {
        info!("No .env file found at %s", root);
    }
}
