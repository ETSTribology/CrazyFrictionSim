use serde::{Deserialize, Serialize};
use super::geometry::Geometry;
use super::contact::Contact;
use super::time_settings::TimeSettings;
use super::space::Space;
use super::boundary_conditions::BoundaryConditions;
use super::materials::Materials;
use super::solver::Solver;
use super::output::Output;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationData {
    pub common: Option<String>,
    pub geometry: Option<Vec<Geometry>>,
    pub contact: Option<Contact>,
    pub time: Option<TimeSettings>,
    pub space: Option<Space>,
    pub boundary_conditions: Option<BoundaryConditions>,
    pub materials: Option<Materials>,
    pub solver: Option<Solver>,
    pub output: Option<Output>,
}

impl SimulationData {
    /// Validate the entire simulation configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate materials
        if let Some(materials) = &self.materials {
            for material in &materials.materials {
                if material.name.trim().is_empty() {
                    return Err("Material name cannot be empty.".to_string());
                }
                // Add more material validations if necessary
            }
        } else {
            return Err("No materials defined in the configuration.".to_string());
        }

        // Validate geometry-material assignments
        if let Some(geometries) = &self.geometry {
            if let Some(materials) = &self.materials {
                let material_names: Vec<&String> = materials.materials.iter().map(|m| &m.name).collect();
                for geometry in geometries {
                    if !material_names.contains(&&geometry.material) {
                        return Err(format!(
                            "Geometry '{}' references undefined material '{}'",
                            geometry.mesh, geometry.material
                        ));
                    }
                }
            } else {
                return Err("No materials defined in the configuration.".to_string());
            }
        }

        // Validate boundary conditions
        if let Some(boundary_conditions) = &self.boundary_conditions {
            boundary_conditions.validate()?;
        }

        // Add more validations for other fields if necessary

        Ok(())
    }
}
