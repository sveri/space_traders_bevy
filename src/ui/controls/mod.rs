mod systems;
pub(crate) mod components;

use bevy::prelude::*;

pub(super) struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (systems::update::player_camera_control, systems::update::mouse_click_handler));
    }
}