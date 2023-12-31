pub(crate) mod client;
pub(crate) mod components;
pub(crate) mod systems;

use bevy::prelude::*;

use self::systems::{
    events::{handle_ship_selected_event, ShipSelected, update_ship_description},
    startup::add_ships,
    update::update_ships,
};

pub(crate) struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ships)
            .add_event::<ShipSelected>()
            .add_systems(Update, update_ships)
            .add_systems(Update, (handle_ship_selected_event.run_if(on_event::<ShipSelected>()), update_ship_description.run_if(on_event::<ShipSelected>())));
    }
}
