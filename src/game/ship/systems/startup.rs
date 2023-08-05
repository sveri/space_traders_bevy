use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, prelude::{RaycastPickTarget, Pointer, On, Click}};

use crate::game::ship::{client::fetch_my_ships, components::Ship};

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
                transform: Transform::from_translation(Vec3::new(ship.get_position().x, ship.get_position().y, 0.)),
                ..default()
            },
            // ship.to_owned(),
            // PbrBundle::default(),
            PickableBundle::default(),    // <- Makes the mesh pickable.
            RaycastPickTarget::default(), // <- Needed for the raycast backend.
            On::<Pointer<Click>>::target_component_mut::<Ship>(|click, ship_rep| {
                println!("{:?}", ship_rep);
            }),
        ));
    })
}
