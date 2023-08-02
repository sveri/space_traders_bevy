use bevy::prelude::*;

use super::{client::fetch_my_ships, components::ShipComponent};



pub(super) fn add_ships(mut commands: Commands) {
    let ships = fetch_my_ships();
    ships.iter().for_each(|s| {
        commands.spawn((s.to_owned(), ShipComponent));
    })
}