//! A module containing compute_edge_combined_degree methods definition.

use polars::prelude::{LazyFrame, Series};

/// Compute the combined degree for each edge in a graph.
pub fn compute_edge_combined_degree(
    edge_df: LazyFrame,
    node_degree_df: LazyFrame,
    node_name_column: &str,
    node_degree_column: &str,
    edge_source_column: &str,
    edge_target_column: &str,
) -> Series {
    fn join_to_degree(df: LazyFrame, column: &str) -> LazyFrame {
        let degree_column = _degree_colname(column);
        let mut result = df.merge(
            node_degree_df.rename(
                columns={node_name_column: column, node_degree_column: degree_column}
            ),
            on=column,
            how="left",
        );
        result[degree_column] = result[degree_column].fillna(0);
        result
    }

    let output_df = join_to_degree(edge_df, edge_source_column);
    let output_df = join_to_degree(output_df, edge_target_column);
    output_df["combined_degree"] = (
        output_df[_degree_colname(edge_source_column).as_str()]
        + output_df[_degree_colname(edge_target_column).as_str()]
    );
    output_df["combined_degree"]
}

fn _degree_colname(column: &str) -> String {
    format!("{column}_degree")
}
