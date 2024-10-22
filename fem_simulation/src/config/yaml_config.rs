use super::SimulationConfig;
use serde_yaml;
use std::fs;

pub fn load_config(file_path: &str) -> SimulationConfig {
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    serde_yaml::from_str(&content).expect("YAML was not well-formatted")
}