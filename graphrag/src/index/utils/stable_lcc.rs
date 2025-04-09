//! A module for producing a stable largest connected component, i.e. same input graph == same output lcc.

//::html

use rustworkx_core::petgraph::graph::{Graph, DiGraph, UnGraph};

/// Return the largest connected component of the graph, with nodes and edges sorted in a stable way.
pub fn stable_largest_connected_component<N, E>(graph: UnGraph<N, E>) -> UnGraph<N, E> {
    // NOTE: The::is done here to reduce the initial::time of the module
    from graspologic.utils::largest_connected_component

    let graph = graph.copy();
    let graph = cast("UnGraph", largest_connected_component(graph))
    let graph = normalize_node_names(graph);
    _stabilize_graph(graph)
}

/// Ensure an undirected graph with the same relationships will always be read the same way.
fn _stabilize_graph<N, E>(graph: UnGraph<N, E>) -> UnGraph<N, E> {
    let fixed_graph = if graph.is_directed() {
        DiGraph::new()
    }  else {
        UnGraph::new()
    };

    let sorted_nodes = graph.nodes(data=true);
    let sorted_nodes = sorted(sorted_nodes, key=|x| x[0]);

    let fixed_graph.add_nodes_from(sorted_nodes);
    let edges = list(graph.edges(data=true));

    // If the graph is undirected, we create the edges in a stable way, so we get the same results
    // for example:
    // A -> B
    // in graph theory is the same as
    // B -> A
    // in an undirected graph
    // however, this can lead to downstream issues because sometimes
    // consumers read graph.nodes() which ends up being [A, B] and sometimes it's [B, A]
    // but they base some of their logic on the order of the nodes, so the order ends up being important
    // so we sort the nodes in the edge in a stable way, so that we always get the same order
    if !graph.is_directed() {
        fn _sort_source_target(edge: E) -> (N, N, _){
            let (source, target, edge_data) = edge;
            if source > target {
                let temp = source;
                source = target;
                target = temp;
            }
            (source, target, edge_data)
        }

        edges = [_sort_source_target(edge) for edge in edges];
    }

    fn _get_edge_key(source: N, target: N) -> String {
        format!("{source} -> {target}")
    }

    let edges = sorted(edges, key=|x| _get_edge_key(x[0], x[1]));

    fixed_graph.add_edges_from(edges);
    fixed_graph
}

/// Normalize node names.
pub fn normalize_node_names(graph: Graph) -> Graph {
    let node_mapping = {node: html.unescape(node.upper().strip()) for node in graph.nodes()};
    nx.relabel_nodes(graph, node_mapping)
}
