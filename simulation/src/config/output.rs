use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParaviewOptions {
    pub material: Option<bool>,
    pub body_ids: Option<bool>,
    pub tensor_values: Option<bool>,
    pub nodes: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParaviewOutput {
    pub file_name: Option<String>,
    pub options: Option<ParaviewOptions>,
    pub vismesh_rel_area: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub paraview: Option<ParaviewOutput>,
}
