use serde::{Deserialize, Serialize};
use super::material::Material;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Material {
    pub name: String,
    pub density: f64,          // kg/m³
    pub youngs_modulus: f64,   // GPa
    pub poisson_ratio: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Materials {
    pub materials: Vec<Material>,
}

impl Materials {
    /// Retrieve a material by name.
    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.iter().find(|m| m.name == name)
    }
}
