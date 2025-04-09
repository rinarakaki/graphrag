//! Parameterization settings for the default configuration.

/// Configuration section for snapshots.
pub struct SnapshotsConfig {
    /// A flag indicating whether to take snapshots of embeddings.
    pub embeddings: bool,

    /// A flag indicating whether to take snapshots of GraphML.
    pub graphml: bool,
}

impl Default for SnapshotsConfig {
    /// Default values for snapshots.
    fn default() -> Self {
        SnapshotsConfig {
            embeddings: false,
            graphml: false,
        }
    }
}
