use super::SimulationConfig;
use serde_json;
use std::fs;

pub fn load_config(file_path: &str) -> SimulationConfig {
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&content).expect("JSON was not well-formatted")
}