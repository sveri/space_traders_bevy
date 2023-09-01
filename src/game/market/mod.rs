pub(crate) mod components;
pub(crate) mod events;
pub(crate) mod update;

use bevy::prelude::*;

use crate::ui::controls::systems::update::MPressedEvent;

use self::{
    events::{handle_get_market_clicked_for_ship_event, GetMarketAtShipLocationEvent},
    update::show_hide_market,
};

pub(crate) struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GetMarketAtShipLocationEvent>().add_systems(
            Update,
            (
                show_hide_market.run_if(on_event::<MPressedEvent>()),
                handle_get_market_clicked_for_ship_event.run_if(on_event::<GetMarketAtShipLocationEvent>()),
            ),
        );
    }
}
