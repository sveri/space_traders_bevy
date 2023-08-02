pub(crate) mod client;
pub(crate) mod components;
pub(crate) mod systems;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use self::systems::{update::show_ships, startup::add_ships};


pub(crate) struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ships)
            .add_systems(Update, show_ships.run_if(on_timer(Duration::from_secs_f64(1.0))));
    }
}
