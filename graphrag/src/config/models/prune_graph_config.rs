//! Parameterization settings for the default configuration.

/// Configuration section for pruning graphs.
pub struct PruneGraphConfig {
    /// The minimum node frequency to allow.
    pub min_node_freq: usize,

    /// The maximum standard deviation of node frequency to allow.
    pub max_node_freq_std: Option<f64>,

    /// The minimum node degree to allow.
    pub min_node_degree: usize,

    /// The maximum standard deviation of node degree to allow.
    pub max_node_degree_std: Option<f64>,

    /// The minimum edge weight percentile to allow. Use e.g, `40` for 40%.
    pub min_edge_weight_pct: f64,

    /// Remove ego nodes.
    pub remove_ego_nodes: bool,

    /// Only use largest connected component.
    pub lcc_only: bool,
}

impl Default for PruneGraphConfig {
    /// Default values for pruning graph.
    fn default() -> Self {
        PruneGraphConfig {
            min_node_freq: 2,
            max_node_freq_std: None,
            min_node_degree: 1,
            max_node_degree_std: None,
            min_edge_weight_pct: 40.0,
            remove_ego_nodes: false,
            lcc_only: false,
        }
    }
}
