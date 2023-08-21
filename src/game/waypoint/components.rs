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
    traits: Vec<WaypointTrait>,
}

impl Waypoint {
    pub(crate) fn get_display_size(&self) -> f32 { 5. }

    pub(crate) fn get_traits(&self) -> String {
        let ts = &self.traits;
        ts.iter().map(|t| t.name.clone()).collect::<Vec<String>>().join(", ")
    }

    pub(crate) fn has_marketplace(&self) -> bool {
        self.traits.iter().any(|t| t.symbol == WaypointTraitSymbol::MARKETPLACE)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct WaypointTrait {
    name: String,
    symbol: WaypointTraitSymbol,
}



#[derive(Debug, Deserialize, Clone, PartialEq)]
pub(crate) enum WaypointTraitSymbol {
    UNCHARTED,
    MARKETPLACE,
    SHIPYARD,
    OUTPOST,
    TOXIC_ATMOSPHERE,
    VOLCANIC,
    WEAK_GRAVITY,
    OVERCROWDED,
    HIGH_TECH,
    BUREAUCRATIC,
    TEMPERATE,
    BARREN,
    FROZEN,
    MINERAL_DEPOSITS,
    COMMON_METAL_DEPOSITS,
    STRIPPED,
    VIBRANT_AURORAS,
    STRONG_MAGNETOSPHERE,
    MILITARY_BASE,
    DRY_SEABEDS,

}

impl Display for WaypointTraitSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WaypointTraitSymbol::DRY_SEABEDS => write!(f, "DRY_SEABEDS"),
            WaypointTraitSymbol::MILITARY_BASE => write!(f, "MILITARY_BASE"),
            WaypointTraitSymbol::STRONG_MAGNETOSPHERE => write!(f, "STRONG_MAGNETOSPHERE"),
            WaypointTraitSymbol::VIBRANT_AURORAS => write!(f, "VIBRANT_AURORAS"),
            WaypointTraitSymbol::STRIPPED => write!(f, "STRIPPED"),
            WaypointTraitSymbol::COMMON_METAL_DEPOSITS => write!(f, "COMMON_METAL_DEPOSITS"),
            WaypointTraitSymbol::MINERAL_DEPOSITS => write!(f, "MINERAL_DEPOSITS"),
            WaypointTraitSymbol::FROZEN => write!(f, "FROZEN"),
            WaypointTraitSymbol::BARREN => write!(f, "BARREN"),
            WaypointTraitSymbol::TEMPERATE => write!(f, "TEMPERATE"),
            WaypointTraitSymbol::UNCHARTED => write!(f, "UNCHARTED"),
            WaypointTraitSymbol::MARKETPLACE => write!(f, "MARKETPLACE"),
            WaypointTraitSymbol::SHIPYARD => write!(f, "SHIPYARD"),
            WaypointTraitSymbol::OUTPOST => write!(f, "OUTPOST"),
            WaypointTraitSymbol::TOXIC_ATMOSPHERE => write!(f, "Toxic_Atmosphere"),
            WaypointTraitSymbol::VOLCANIC => write!(f, "VOLCANIC"),
            WaypointTraitSymbol::WEAK_GRAVITY => write!(f, "WEAK_GRAVITY"),
            WaypointTraitSymbol::OVERCROWDED => write!(f, "OVERCROWDED"),
            WaypointTraitSymbol::HIGH_TECH => write!(f, "HIGH_TECH"),
            WaypointTraitSymbol::BUREAUCRATIC => write!(f, "BUREAUCRATIC"),
        }
    }
}