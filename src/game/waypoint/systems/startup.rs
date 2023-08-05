use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle, prelude::{RaycastPickTarget, Pointer, On, Click}};

use crate::{st_client, game::waypoint::components::Waypoint};

pub(crate) fn add_waypoints(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    let agent_details = st_client::fetch_agent_details();
    let waypoints = st_client::fetch_waypoints(agent_details.get_headquarter_system_symbol().as_str());
    waypoints.iter().for_each(|waypoint| {
        let mut planet_transform = Transform::from_xyz(waypoint.x, waypoint.y, 0.0);
        planet_transform.scale = Vec3::new(0.01, 0.01, 0.01);
        commands.spawn((
            waypoint.to_owned(),
            SpriteBundle {
                transform: planet_transform,
                texture: asset_server.load("planets/planet02.png"),
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Click>>::target_component_mut::<Waypoint>(|click, waypoint_rep| {
                println!("{:?}", waypoint_rep);
            }),
        ));
    })
}
