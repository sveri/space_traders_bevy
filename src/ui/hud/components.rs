use bevy::prelude::*;


#[derive(Resource)]
pub(crate) struct PlanetUpdateTimer(pub Timer);

#[derive(Resource)]
pub(crate) struct ShipUpdateTimer(pub Timer);

#[derive(Component)]
struct ShipRepresentation;

#[derive(Component)]
struct WaypointRepresentation;

#[derive(Component)]
struct AgentDetailsText;
