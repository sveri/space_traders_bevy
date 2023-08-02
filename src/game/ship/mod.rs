pub(crate) mod components;
pub(crate) mod client;
pub(crate) mod startup;

use bevy::prelude::*;

use self::startup::add_ships;



pub(crate) struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ships);
    }
}