pub(crate) mod hud;
pub(crate) mod hud_buttons;
mod controls;

use bevy::prelude::*;

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((hud::HudPlugin, hud_buttons::HudButtonsPlugin, controls::ControlsPlugin));
        
    }
}