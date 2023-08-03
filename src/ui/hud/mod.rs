pub(crate) mod components;
pub(crate) mod systems;



use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};


use systems::layout::*;

pub(crate) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (show_agent_details, selected_ship_text, selected_waypoint_text, init_error_text))
        .add_systems(Update, systems::update::clear_error_text.run_if(on_timer(Duration::from_secs_f64(10.0))));
    }
}
