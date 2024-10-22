use wasm_bindgen_test::*;
use fem_simulation::wasm::WasmFEMSolver;
use serde_json::json;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_solver() {
    let config = json!({
        "mesh_file": "mesh_data",
        "time_step": 0.01,
    });

    let solver = WasmFEMSolver::new(&JsValue::from_serde(&config).unwrap()).unwrap();
    solver.run();
    let results = solver.get_results();
    assert!(!results.is_null());
}