use bevy::prelude::*;

use crate::game::ship::components::Ship;

pub(crate) fn update_ships(mut ships: Query<(&mut Transform, &Ship)>) {
    for (mut transform, ship) in ships.iter_mut() {
        transform.translation = Vec3::new(ship.get_position().x, ship.get_position().y, 0.)
    }    
}
