pub(crate) mod components;
pub(crate) mod systems;

use bevy::prelude::*;

use self::{
    components::SelectedShip,
    systems::update::{keyboard_input, MPressedEvent},
};

pub(super) struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedShip(None))
            .add_event::<MPressedEvent>()
            .add_systems(Update, keyboard_input);
    }
}
