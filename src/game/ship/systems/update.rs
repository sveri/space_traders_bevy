use bevy::prelude::*;

use crate::game::ship::components::ShipRepresentation;



pub(crate) fn show_ships(mut commands: Commands, ships: Query<&crate::game::ship::components::Ship>) {
    for ship in ships.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(ship.get_display_size().0, ship.get_display_size().1)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(ship.nav.route.departure.x, ship.nav.route.departure.y, 0.)),
                ..default()
            },
            ShipRepresentation,
        ));
    }
}