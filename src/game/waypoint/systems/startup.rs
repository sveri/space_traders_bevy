use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, prelude::{RaycastPickTarget, Pointer, On, Click}};

use crate::{st_client, game::waypoint::components::Waypoint};

use super::events::WaypointSelected;

pub(crate) fn add_waypoints(mut commands: Commands, asset_server: Res<AssetServer>) {
    let agent_details = st_client::fetch_agent_details();
    let waypoints = st_client::fetch_waypoints(agent_details.get_headquarter_system_symbol().as_str()).unwrap();
    waypoints.iter().for_each(|waypoint| {
        commands.spawn((
            waypoint.to_owned(),
            SpriteBundle {
                transform: Transform::from_xyz(waypoint.x, waypoint.y, 0.0),
                texture: asset_server.load("planets/planet02.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Click>>::send_event::<WaypointSelected>(),
        ));
    })
}
