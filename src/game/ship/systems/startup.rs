use bevy::prelude::*;

use crate::game::ship::{ client::fetch_my_ships};




pub(crate) fn add_ships(mut commands: Commands) {
    let ships = fetch_my_ships();
    ships.iter().for_each(|s| {
        commands.spawn((s.to_owned()));
    })
}