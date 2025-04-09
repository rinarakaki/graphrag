//! A module containing 'NodePosition' model.

/// Node position class definition.
pub struct NodePosition {
    label: String,
    cluster: String,
    size: f64,

    x: f64,
    y: f64,
    z: Option<f64>,
}

impl NodePosition {
    /// To pandas method definition.
    pub fn to_pandas(&self) -> (String, f64, f64, String, f64) {
        (self.label.clone(), self.x, self.y, self.cluster.clone(), self.size)
    }
}

pub type GraphLayout = Vec<NodePosition>;
