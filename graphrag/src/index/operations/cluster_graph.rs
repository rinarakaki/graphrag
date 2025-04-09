//! A module containing cluster_graph, apply_clustering and run_layout methods definition.

use std::collections::HashMap;

use log::warn;

use rustworkx_core::petgraph::graph::{Graph, UnGraph};

use crate::index::utils::stable_lcc::stable_largest_connected_component;

pub type Communities = Vec<(int, int, int, Vec<String>)>;

/// Apply a hierarchical clustering algorithm to a graph.
pub fn cluster_graph(
    graph: UnGraph,
    max_cluster_size: usize,
    use_lcc: bool,
    seed: Option<usize>,
) -> Communities {
    if graph.nodes.len() == 0 {
        warn("Graph has no nodes");
        return Vec::new();
    }

    let (node_id_to_community_map, parent_mapping) = _compute_leiden_communities(
        graph,
        max_cluster_size,
        use_lcc,
        seed,
    );

    let levels = sorted(node_id_to_community_map.keys());

    let mut clusters = HashMap::<usize, HashMap<usize, Vec<String>>>::new();
    for level in levels {
        let mut result = HashMap::new();
        clusters[level] = result;
        for (node_id, raw_community_id) in node_id_to_community_map[level].items() {
            let community_id = raw_community_id;
            if !result.contains_key(community_id) {
                result[community_id] = Vec::new();
            }
            result[community_id].push(node_id);
        }
    }

    let mut results: Communities = Vec::new();
    for level in clusters {
        for (cluster_id, nodes) in clusters[level].items() {
            results.push((level, cluster_id, parent_mapping[cluster_id], nodes));
        }
    }
    results
}

// Taken from graph_intelligence & adapted
/// Return Leiden root communities and their hierarchy mapping.
fn _compute_leiden_communities(
    mut graph: Graph,
    max_cluster_size: u32,
    use_lcc: bool,
    seed: Option<u64>,
) -> (HashMap<int, HashMap<str, int>>, HashMap<int, int>) {
    use network_partitions::leiden::hierarchical_leiden;

    if use_lcc {
        graph = stable_largest_connected_component(graph);
    }

    let community_mapping = hierarchical_leiden(
        graph,
        None,
        Some(0),
        Some(1.0),
        Some(0.001),
        true,
        max_cluster_size,
        seed,
    )?;
    let results = HashMap::<int, HashMap<str, int>>::new();
    let hierarchy = HashMap::<int, int>::new();
    for partition in community_mapping.iter() {
        results[partition.level] = results.get(partition.level, {});
        results[partition.level][partition.node] = partition.cluster;

        lhierarchy[partition.cluster] = (
            if let Some(parent_cluster) = partition.parent_cluster { parent_cluster } else { -1 }
        );
    }

    (results, hierarchy)
}
