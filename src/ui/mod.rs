pub(super) mod hud;
pub(super) mod hud_buttons;
pub(super) mod controls;

use bevy::prelude::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((hud::HudPlugin, hud_buttons::HudButtonsPlugin, controls::ControlsPlugin));
        
    }
}