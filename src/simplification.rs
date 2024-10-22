use std::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Simplification {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "slight")]
    Slight,

    #[serde(rename = "medium")]
    Medium,

    #[serde(rename = "moderate")]
    Moderate,

    #[serde(rename = "aggressive")]
    Aggressive,

    #[serde(rename = "max")]
    Max,
}

impl fmt::Display for Simplification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Simplification::None => write!(f, "none"),
            Simplification::Slight => write!(f, "slight"),
            Simplification::Medium => write!(f, "medium"),
            Simplification::Moderate => write!(f, "moderate"),
            Simplification::Aggressive => write!(f, "aggressive"),
            Simplification::Max => write!(f, "max"),
        }
    }
}

pub struct SimplifiedBorders {
    pub none: (usize, geo::MultiPolygon),
    pub slight: (usize, geo::MultiPolygon),
    pub medium: (usize, geo::MultiPolygon),
    pub moderate: (usize, geo::MultiPolygon),
    pub aggressive: (usize, geo::MultiPolygon),
    pub max: (usize, geo::MultiPolygon),
}

impl SimplifiedBorders {
    pub fn sizes(&self) -> (usize, usize, usize, usize, usize, usize) {
        (
            self.none.0,
            self.slight.0,
            self.medium.0,
            self.moderate.0,
            self.aggressive.0,
            self.max.0,
        )
    }
}
