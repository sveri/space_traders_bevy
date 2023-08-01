pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::layout::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (get_agent_details, selected_ship_text, selected_waypoint_text));
        
    }
}