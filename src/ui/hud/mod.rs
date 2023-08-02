pub(crate) mod components;
pub(crate) mod systems;



use bevy::prelude::*;


use systems::layout::*;

pub(crate) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (show_agent_details, selected_ship_text, selected_waypoint_text, show_error_text));
    }
}
