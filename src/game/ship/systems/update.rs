use bevy::prelude::*;

use crate::{game::ship::components::ShipRepresentation, st_client::Waypoint};



pub(crate) fn show_ships(mut commands: Commands, ships: Query<&crate::game::ship::components::Ship>, waypoints: Query<&Waypoint>) {
    for ship in ships.iter() {
        // if let Some(ship_waypoint) = waypoints.iter().find(|waypoint| waypoint.symbol == ship.nav.waypoint_symbol){
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
                ShipRepresentation,
            ));
        // }
    }
}