pub mod yaml_config;
pub mod json_config;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimulationConfig {
    pub mesh_file: String,
    pub time_step: f64,
    // ... other config parameters
}