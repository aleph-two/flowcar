use std::fmt;

use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Type))]
#[cfg_attr(
    feature = "ssr",
    sqlx(type_name = "vehicle_transmission", rename_all = "lowercase")
)]
pub enum Transmission {
    Manual,
    Automatic,
}

impl fmt::Display for Transmission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Transmission::Manual => write!(f, "Manual"),
            Transmission::Automatic => write!(f, "Automatic"),
        }
    }
}

impl Transmission {
    pub fn values() -> Vec<Transmission> {
        vec![Transmission::Manual, Transmission::Automatic]
    }
}
