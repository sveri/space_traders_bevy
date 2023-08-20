pub(crate) mod components;
pub(crate) mod systems;

use bevy::prelude::*;

use crate::ui::hud_buttons::systems::layout::setup_buttons;

use self::systems::events::{handle_dock_clicked_event, handle_orbit_clicked_event, DockClicked, OrbitClicked, MoveShipClicked, handle_move_ship, GetMarketClicked, handle_get_market_clicked};

pub(crate) struct HudButtonsPlugin;

impl Plugin for HudButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_buttons)
            .add_event::<OrbitClicked>()
            .add_event::<DockClicked>()
            .add_event::<MoveShipClicked>()
            .add_event::<GetMarketClicked>()
            .add_systems(
                Update,
                (
                    handle_orbit_clicked_event.run_if(on_event::<OrbitClicked>()),
                    handle_dock_clicked_event.run_if(on_event::<DockClicked>()),
                    handle_move_ship.run_if(on_event::<MoveShipClicked>()),
                    handle_get_market_clicked.run_if(on_event::<GetMarketClicked>()),
                ),
            );
    }
}
