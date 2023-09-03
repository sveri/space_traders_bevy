use bevy::prelude::*;
use bevy_mod_picking::{
    prelude::{Click, On, Pointer, RaycastPickTarget},
    PickableBundle,
};

use crate::{game::waypoint::components::WaypointType, st_client};

use super::events::WaypointSelected;

pub(crate) fn add_waypoints(mut commands: Commands, asset_server: Res<AssetServer>) {
    let agent_details = st_client::fetch_agent_details();
    let waypoints = st_client::fetch_waypoints(agent_details.get_headquarter_system_symbol().as_str()).unwrap();
    waypoints.iter().for_each(|waypoint| {
        let (asset, sprite_size) = match waypoint.waypoint_type {
            WaypointType::JUMP_GATE => (asset_server.load("other/jump_gate.png"), Some(Vec2::new(3.0, 3.0))),
            WaypointType::ORBITAL_STATION => (asset_server.load("other/space_station.png"), Some(Vec2::new(3.0, 3.0))),
            WaypointType::ASTEROID_FIELD => (asset_server.load("other/asteroids.png"), Some(Vec2::new(4.0, 4.0))),

            _ => (asset_server.load("planets/planet09.png"), Some(Vec2::new(7.0, 7.0))),
        };

        commands.spawn((
            waypoint.to_owned(),
            SpriteBundle {
                transform: Transform::from_xyz(waypoint.get_position().x, waypoint.get_position().y, 0.0),
                texture: asset,
                sprite: Sprite {
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Click>>::send_event::<WaypointSelected>(),
            Name::new(format!("Waypoint {}", waypoint.symbol)),
        ));
    })
}
