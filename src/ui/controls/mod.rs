pub(crate) mod components;
pub(crate) mod systems;

use bevy::prelude::*;

use self::{
    components::SelectedShip,
    systems::update::{keyboard_input, player_camera_control, MPressedEvent},
};

pub(super) struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedShip(None))
            .add_event::<MPressedEvent>()
            .add_systems(Update, (player_camera_control, keyboard_input));
    }
}
