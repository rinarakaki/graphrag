//! Parameterization settings for the default configuration.

/// The default configuration section for Node2Vec.
pub struct EmbedGraphConfig {
    /// A flag indicating whether to enable node2vec.
    pub enabled: bool,

    /// The node2vec vector dimensions.
    pub dimensions: usize,

    /// The node2vec number of walks.
    pub num_walks: usize,

    /// The node2vec walk length.
    pub walk_length: usize,

    /// The node2vec window size.
    pub window_size: usize,

    /// The node2vec iterations.
    pub iterations: usize,

    /// The node2vec random seed.
    pub random_seed: usize,

    /// Whether to use the largest connected component.
    pub use_lcc: bool,
}

impl Default for EmbedGraphConfig {
    /// Default values for embedding graph.
    fn default() -> Self {
        EmbedGraphConfig {
            enabled: false,
            dimensions: 1536,
            num_walks: 10,
            walk_length: 40,
            window_size: 2,
            iterations: 3,
            random_seed: 597832,
            use_lcc: true,
        }
    }
}
