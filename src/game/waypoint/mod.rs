pub(crate) mod components;
pub(super) mod systems;

use bevy::prelude::*;

use self::systems::{
    events::{handle_waypoint_selected_event, WaypointSelected},
    startup::add_waypoints,
};

pub(crate) struct WaypointPlugin;

impl Plugin for WaypointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_waypoints)
            .add_event::<WaypointSelected>()
            .add_systems(Update, handle_waypoint_selected_event.run_if(on_event::<WaypointSelected>()));
    }
}
