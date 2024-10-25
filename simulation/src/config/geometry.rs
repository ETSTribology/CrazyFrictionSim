use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transformation {
    pub translation: Option<[f64; 3]>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Geometry {
    pub mesh: String,
    pub transformation: Option<Transformation>,
    pub volume_selection: Option<u32>,
    pub is_obstacle: Option<bool>,
    pub material: String, // Name of the material assigned
}
