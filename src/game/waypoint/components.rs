

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
    traits: Vec<WaypointTrait>,
}

impl Waypoint {
    pub(crate) fn get_display_size(&self) -> f32 {
        5.
    }

    pub(crate) fn get_traits(&self) -> String {
        let ts = &self.traits;
        ts.iter().map(|t| t.name.clone()).collect::<Vec<String>>().join(", ")
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct WaypointTrait {
    name: String,
}