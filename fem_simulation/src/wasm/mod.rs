use wasm_bindgen::prelude::*;
use crate::fem::FEMSolver;
use crate::config::SimulationConfig;

#[wasm_bindgen]
pub struct WasmFEMSolver {
    solver: FEMSolver,
}

#[wasm_bindgen]
impl WasmFEMSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(config_data: &JsValue) -> Result<WasmFEMSolver, JsValue> {
        // Deserialize config_data from JavaScript
        let config: SimulationConfig = config_data.into_serde()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Create a new FEMSolver instance
        Ok(WasmFEMSolver {
            solver: FEMSolver::new(&config),
        })
    }

    pub fn run(&mut self) {
        // Run the simulation
        self.solver.run();
    }

    pub fn get_results(&self) -> JsValue {
        // Serialize results to JsValue
        JsValue::from_serde(&self.solver.get_results())
            .unwrap_or(JsValue::NULL)
    }
}
