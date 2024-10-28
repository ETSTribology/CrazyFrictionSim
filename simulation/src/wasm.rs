use wasm_bindgen::prelude::*;
use crate::config::{
    simulation_data::SimulationData,
    boundary_conditions::BoundaryConditions,
    boundary_condition_type::BoundaryConditionType,
};
use serde_json;

/// Expose the SimulationData struct to WebAssembly
#[wasm_bindgen]
pub struct WasmSimulationData {
    data: SimulationData,
}

#[wasm_bindgen]
impl WasmSimulationData {
    /// Function to parse JSON input and create a new SimulationData instance
    #[wasm_bindgen(constructor)]
    pub fn new_from_json(input: &str) -> Result<WasmSimulationData, JsValue> {
        match serde_json::from_str::<SimulationData>(input) {
            Ok(data) => Ok(WasmSimulationData { data }),
            Err(err) => Err(JsValue::from_str(&format!("Failed to parse JSON: {}", err))),
        }
    }

    /// Get the JSON string of the configuration
    #[wasm_bindgen]
    pub fn get_json(&self) -> Result<String, JsValue> {
        match serde_json::to_string(&self.data) {
            Ok(json_str) => Ok(json_str),
            Err(err) => Err(JsValue::from_str(&format!("Failed to serialize to JSON: {}", err))),
        }
    }

    /// Access boundary conditions
    #[wasm_bindgen]
    pub fn get_boundary_conditions(&self) -> Result<JsValue, JsValue> {
        match &self.data.boundary_conditions {
            Some(bc) => Ok(JsValue::from_serde(bc).unwrap()),
            None => Err(JsValue::from_str("No boundary conditions defined")),
        }
    }

    /// Validate simulation data
    #[wasm_bindgen]
    pub fn validate(&self) -> Result<(), JsValue> {
        match self.data.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }
}
