pub(crate) mod components;
pub(crate) mod systems;

use bevy::prelude::*;

use crate::ui::hud_buttons::systems::layout::setup_buttons;
use crate::ui::hud_buttons::systems::update::update_button_system;

pub(crate) struct HudButtonsPlugin;

impl Plugin for HudButtonsPlugin {
    fn build(&self, app: &mut App) { app.add_systems(Startup, setup_buttons).add_systems(Update, update_button_system); }
}
