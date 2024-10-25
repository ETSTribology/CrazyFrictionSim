use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSettings {
    pub integrator: Option<String>,
    pub tend: Option<f64>,
    pub dt: Option<f64>,
}
