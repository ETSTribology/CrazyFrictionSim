mod parser;

use std::error::Error;
use parser::simulation_data::SimulationData;
use parser::{parse_json, parse_yaml, read_file};

fn main() -> Result<(), Box<dyn Error>> {
    // Replace with your actual file paths
    let json_file_path = "path/to/your/input.json";
    let yaml_file_path = "path/to/your/input.yaml";

    // Read JSON file
    let json_content = read_file(json_file_path)?;
    let json_data = parse_json(&json_content)?;
    println!("Parsed JSON Data: {:?}", json_data);

    // Read YAML file
    let yaml_content = read_file(yaml_file_path)?;
    let yaml_data = parse_yaml(&yaml_content)?;
    println!("Parsed YAML Data: {:?}", yaml_data);

    Ok(())
}
