pub(crate) mod components;
pub(crate) mod systems;

use std::time::Duration;

use bevy::prelude::*;

use bevy::time::common_conditions::on_timer;
use systems::layout::*;
use systems::update::*;

pub(crate) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (get_agent_details, selected_ship_text, selected_waypoint_text)).add_systems(
            Update,
            (
                show_waypoints.run_if(on_timer(Duration::from_secs_f64(2.0))),
                show_ships.run_if(on_timer(Duration::from_secs_f64(1.0))),
            ),
        );
    }
}
