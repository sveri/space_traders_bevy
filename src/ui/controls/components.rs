use bevy::prelude::*;

use crate::game::{ship::components::Ship, waypoint::components::Waypoint};


#[derive(Component)]
pub(crate) struct MainCamera;

#[derive(Component)]
pub(crate) struct SelectedWaypointText;

#[derive(Resource, Debug, bevy::prelude::Deref)]
pub(crate) struct SelectedShip(pub(crate) Option<Entity>);

// impl std::ops::Deref for SelectedShip {
//     type Target = Option<Entity>;

//     fn deref(&self) -> &Self::Target { &self.0 }
// }

#[derive(Component, Debug)]
pub(crate) struct SelectedWaypoint {
    pub(crate)  waypoint: Waypoint,
}