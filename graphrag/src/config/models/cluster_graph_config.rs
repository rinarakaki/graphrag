//! Parameterization settings for the default configuration.

/// Configuration section for clustering graphs.
pub struct ClusterGraphConfig {
    /// The maximum cluster size to use.
    pub max_cluster_size: usize,

    /// Whether to use the largest connected component.
    pub use_lcc: bool,

    /// The seed to use for the clustering.
    pub seed: usize,
}

impl Default for ClusterGraphConfig {
    /// Default values for cluster graph.
    fn default() -> Self {
        ClusterGraphConfig {
            max_cluster_size: 10,
            use_lcc: true,
            seed: 0xDEADBEEF,
        }
    }
}
