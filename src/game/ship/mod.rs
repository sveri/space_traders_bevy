pub(crate) mod client;
pub(crate) mod components;
pub(crate) mod systems;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use self::systems::{
    events::{handle_ship_selected_event, ShipSelected},
    startup::add_ships,
    update::update_ships,
};

pub(crate) struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ships)
            .add_event::<ShipSelected>()
            .add_systems(Update, update_ships.run_if(on_timer(Duration::from_secs_f64(1.0))))
            .add_systems(Update, handle_ship_selected_event.run_if(on_event::<ShipSelected>()));
    }
}
