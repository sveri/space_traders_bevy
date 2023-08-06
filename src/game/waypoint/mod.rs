pub(crate) mod components;
pub(super) mod systems;

use bevy::prelude::*;

use self::systems::{startup::add_waypoints, events::{handle_waypoint_selected_event, WaypointSelected}};

pub(crate) struct WaypointPlugin;

impl Plugin for WaypointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_waypoints)
        .add_systems(Update, handle_waypoint_selected_event)
        .add_event::<WaypointSelected>();
    }
}
