pub(crate) mod components;
pub(super) mod systems;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use self::systems::update::show_waypoints;
use self::systems::startup::add_waypoints;

pub(crate) struct WaypointPlugin;

impl Plugin for WaypointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_waypoints) 
            .add_systems(Update, show_waypoints.run_if(on_timer(Duration::from_secs_f64(2.0))));
    }
}
