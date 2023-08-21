pub(crate) mod components;
pub(crate) mod market;
pub(super) mod ship;
pub(super) mod waypoint;


use bevy::prelude::*;

use self::{ship::ShipPlugin, market::MarketPlugin, waypoint::WaypointPlugin};



pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) { app.add_plugins((ShipPlugin, WaypointPlugin, MarketPlugin)); }
}
