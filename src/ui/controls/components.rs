use bevy::prelude::*;

use crate::{game::{ship::components::Ship, waypoint::components::Waypoint}};


#[derive(Component)]
pub(crate) struct MainCamera;

#[derive(Component)]
pub(crate) struct SelectedWaypointText;

#[derive(Component, Debug)]
pub(crate) struct SelectedShip {
    pub(crate) ship: Ship,
}

#[derive(Component, Debug)]
pub(crate) struct SelectedWaypoint {
    pub(crate)  waypoint: Waypoint,
}