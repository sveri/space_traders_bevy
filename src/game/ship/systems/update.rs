use bevy::prelude::*;
use bevy_mod_picking::{prelude::{RaycastPickTarget, Click, Pointer, On}, PickableBundle};

use crate::game::ship::components::Ship;

pub(crate) fn show_ships(mut commands: Commands, ships: Query<&Ship>) {
    for ship in ships.iter() {
        let sym = ship.symbol.clone();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(ship.get_display_size().0, ship.get_display_size().1)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(ship.get_position().x, ship.get_position().y, 0.)),
                ..default()
            },
            PickableBundle::default(),    // <- Makes the mesh pickable.
            RaycastPickTarget::default(), // <- Needed for the raycast backend.
            On::<Pointer<Click>>::target_component_mut::<Ship>(|click, ship_rep| {
                println!("{:?}", ship_rep);
            })
        ));
    }
}
