use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, prelude::{RaycastPickTarget, Pointer, On, Click}};

use crate::game::ship::client::fetch_my_ships;

use super::events::ShipSelected;


pub(crate) fn add_ships(mut commands: Commands) {
    let ships = fetch_my_ships();
    ships.iter().for_each(|ship| {
        commands.spawn((
            ship.to_owned(),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(ship.get_display_size().0, ship.get_display_size().1)),
                    ..default()
                },
                transform: ship.get_transform(),
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Click>>::send_event::<ShipSelected>(),
        ));
    })
}
