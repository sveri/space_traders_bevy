pub(crate) mod components;
pub(crate) mod startup;

use bevy::prelude::*;



pub(crate) struct WaypointPlugin;

impl Plugin for WaypointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup::add_waypoints);
    }
}