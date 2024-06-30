use std::fmt;

use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Type))]
#[cfg_attr(
    feature = "ssr",
    sqlx(type_name = "vehicle_category", rename_all = "lowercase")
)]
pub enum Category {
    Coupe,
    Suv,
    Motorcycle,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Coupe => write!(f, "Coupe"),
            Category::Suv => write!(f, "SUV"),
            Category::Motorcycle => write!(f, "Motorcycle"),
        }
    }
}

impl Category {
    pub fn values() -> Vec<Category> {
        vec![Category::Coupe, Category::Suv, Category::Motorcycle]
    }
}
