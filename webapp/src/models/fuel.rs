use std::fmt;

use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Type))]
#[cfg_attr(
    feature = "ssr",
    sqlx(type_name = "vehicle_fuel", rename_all = "lowercase")
)]
pub enum Fuel {
    Electric,
    Hybrid,
    Diesel,
    Gas,
}

impl fmt::Display for Fuel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fuel::Electric => write!(f, "Electric"),
            Fuel::Hybrid => write!(f, "Hybrid"),
            Fuel::Diesel => write!(f, "Diesel"),
            Fuel::Gas => write!(f, "Gas"),
        }
    }
}

impl Fuel {
    pub fn values() -> Vec<Fuel> {
        vec![Fuel::Electric, Fuel::Hybrid, Fuel::Diesel, Fuel::Gas]
    }
}
