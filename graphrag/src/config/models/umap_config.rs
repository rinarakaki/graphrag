//! Parameterization settings for the default configuration.

/// Configuration section for UMAP.
pub struct UmapConfig {
    /// A flag indicating whether to enable UMAP.
    pub enabled: bool,
}

impl Default for UmapConfig {
    /// Default values for UMAP.
    fn default() -> Self {
        UmapConfig { enabled: false }
    }
}
