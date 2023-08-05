pub(crate) mod components;
pub(super) mod systems;

use bevy::prelude::*;

use self::systems::startup::add_waypoints;

pub(crate) struct WaypointPlugin;

impl Plugin for WaypointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_waypoints);
    }
}
