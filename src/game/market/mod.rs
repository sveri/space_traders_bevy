pub(crate) mod components;
pub(crate) mod layout;
pub(crate) mod update;

use bevy::prelude::*;

use crate::ui::controls::systems::update::MPressedEvent;

use self::{layout::init_market_overview, update::show_hide_market};

pub(crate) struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_market_overview)
            .add_systems(Update, show_hide_market.run_if(on_event::<MPressedEvent>()));
    }
}
