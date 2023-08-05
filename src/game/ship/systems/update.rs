use bevy::prelude::*;

use crate::game::ship::components::Ship;

pub(crate) fn update_ships(mut ships: Query<(&mut Transform, &Ship)>) {
    for (mut transform, ship) in ships.iter_mut() {
        transform.translation = ship.get_position();
    }    
}
