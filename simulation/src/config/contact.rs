use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub friction_coefficient: Option<f64>,
    pub enabled: Option<bool>,
    pub dhat: Option<f64>,
    pub epsv: Option<f64>,
}
