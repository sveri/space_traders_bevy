pub(crate) mod components;
pub(crate) mod systems;

use bevy::prelude::*;

use crate::ui::hud_buttons::systems::layout::setup_buttons;
use crate::ui::hud_buttons::systems::update;

use self::systems::events::{OrbitClicked, handle_orbit_clicked_event};

pub(crate) struct HudButtonsPlugin;

impl Plugin for HudButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_buttons)
            // .add_systems(Update, (update::orbit_clicked, update::move_ship_clicked))
            // .add_systems(Update, handle_orbit_clicked_event)
            .add_event::<OrbitClicked>()
            .add_systems(Update, handle_orbit_clicked_event.run_if(on_event::<OrbitClicked>()));
    }
}
