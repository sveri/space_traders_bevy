pub(super) mod ship;
pub(super) mod waypoint;
pub(crate) mod components;


use bevy::prelude::*;

use self::ship::ShipPlugin;



pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) { app.add_plugins((ShipPlugin, waypoint::WaypointPlugin)); }
}
