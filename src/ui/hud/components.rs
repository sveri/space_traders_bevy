use bevy::prelude::*;


#[derive(Resource)]
pub(crate) struct PlanetUpdateTimer(pub Timer);

#[derive(Resource)]
pub(crate) struct ShipUpdateTimer(pub Timer);

#[derive(Component)]
pub struct ShipRepresentation;

#[derive(Component)]
pub struct WaypointRepresentation;

#[derive(Component)]
pub struct AgentDetailsText;

#[derive(Component)]
pub(crate) struct SelectedShipText;

