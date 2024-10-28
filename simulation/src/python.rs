use pyo3::prelude::*;
use crate::config::{
    simulation_data::SimulationData,
    boundary_conditions::BoundaryConditions,
    boundary_condition_type::BoundaryConditionType,
};
use serde_json;

/// Expose the SimulationData struct to Python
#[pyclass]
pub struct PySimulationData {
    data: SimulationData,
}

#[pymethods]
impl PySimulationData {
    /// Function to parse JSON input and create a new SimulationData instance
    #[new]
    pub fn new_from_json(input: &str) -> PyResult<Self> {
        match serde_json::from_str::<SimulationData>(input) {
            Ok(data) => Ok(PySimulationData { data }),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(format!("Failed to parse JSON: {}", err))),
        }
    }

    /// Get the JSON string of the configuration
    pub fn get_json(&self) -> PyResult<String> {
        match serde_json::to_string(&self.data) {
            Ok(json_str) => Ok(json_str),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize to JSON: {}", err))),
        }
    }

    /// Access contact configuration data
    pub fn get_contact(&self) -> PyResult<Option<&Contact>> {
        Ok(self.data.contact.as_ref())
    }

    /// Access boundary conditions
    pub fn get_boundary_conditions(&self) -> PyResult<Option<&BoundaryConditions>> {
        Ok(self.data.boundary_conditions.as_ref())
    }

    /// Validate simulation data
    pub fn validate(&self) -> PyResult<()> {
        match self.data.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn simulation_config(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySimulationData>()?;
    Ok(())
}
