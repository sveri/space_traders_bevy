

use std::fmt::{self, Display};

use bevy::prelude::*;
use serde::Deserialize;



pub(crate) type Waypoints = Vec<Waypoint>;

#[derive(Debug, Deserialize, Component, Clone)]
pub(crate) struct Waypoint {
    #[serde(alias = "systemSymbol")]
    pub(crate) system_symbol: String,
    pub(crate) symbol: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    // traits: Vec<WaypointTrait>,
    traits: Vec<WaypointTrait>,
}

impl Waypoint {
    pub(crate) fn get_display_size(&self) -> f32 {
        5.
    }

    pub(crate) fn get_traits(&self) -> String {
        let ts = &self.traits;
        ts.iter().map(|t| t.clone().to_string()).collect::<Vec<String>>().join(", ")
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub(crate) enum  WaypointTrait{    
    Uncharted,
    Marketplace,
    Shipyard,
    Outpost,
    Symbol,
}

impl Display for WaypointTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WaypointTrait::Uncharted => write!(f, "Uncharted"),
            WaypointTrait::Marketplace => write!(f, "Marketplace"),
            WaypointTrait::Shipyard => write!(f, "Shipyard"),
            WaypointTrait::Outpost => write!(f, "Outpost"),
            WaypointTrait::Symbol => write!(f, "Symbol"),
        }
    }
}


// impl fmt::Display for WaypointTrait {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             WaypointTrait::Uncharted => write!(f, "Uncharted"),
//         }
//     }
// }
    