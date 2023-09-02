use bevy::prelude::*;
use bevy_mod_picking::{
    prelude::{Click, On, Pointer, RaycastPickTarget},
    PickableBundle,
};

use crate::game::ship::{client::fetch_my_ships, components::ShipState};

use super::events::ShipSelected;

pub(crate) fn add_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ships = fetch_my_ships();
    ships.iter().for_each(|ship| {
        commands.spawn((
            ship.to_owned(),
            SpriteBundle {
                transform: ship.get_transform(),
                texture: asset_server.load("ships/default_frigate.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(3.0, 3.0)),
                    ..default()
                },
                ..default()
            },
            ShipState::new(),
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Click>>::send_event::<ShipSelected>(),
            Name::new(ship.symbol.clone()),
        ));
    })
}
