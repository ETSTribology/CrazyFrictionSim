use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BoundaryConditionType {
    Dirichlet {
        id: u32,
        value: [f64; 3],
    },
    Neumann {
        id: u32,
        force: [f64; 3],
    },
}


use serde::{Deserialize, Serialize};
use super::boundary_condition_type::BoundaryConditionType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoundaryConditions {
    pub conditions: Vec<BoundaryConditionType>,
}

impl BoundaryConditions {
    /// Validate all boundary conditions
    pub fn validate(&self) -> Result<(), String> {
        for condition in &self.conditions {
            match condition {
                BoundaryConditionType::Dirichlet { id, value } => {
                    // Add any validation logic if needed
                    // For example, check if the value is within acceptable range
                },
                BoundaryConditionType::Neumann { id, force } => {
                    // Add validation logic
                },
                BoundaryConditionType::ObstacleDisplacement { id, value } => {
                    // Ensure that expressions are valid, etc.
                },
                // Handle other variants
            }
        }
        Ok(())
    }
}
