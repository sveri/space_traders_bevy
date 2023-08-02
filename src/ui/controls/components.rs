use bevy::prelude::*;

use crate::{ship::Ship, st_client::Waypoint};


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