pub(crate) mod components;
pub(crate) mod systems;

use std::time::Duration;

use bevy::prelude::*;

use bevy::time::common_conditions::on_timer;
use systems::layout::*;

pub(crate) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (get_agent_details, selected_ship_text, selected_waypoint_text));
    }
}
