use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::st_client;

pub(crate) fn add_waypoints(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let agent_details = st_client::fetch_agent_details();
    let waypoints = st_client::fetch_waypoints(agent_details.get_headquarter_system_symbol().as_str());
    waypoints.iter().for_each(|waypoint| {
        commands.spawn((
            waypoint.to_owned(),
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(waypoint.get_display_size()).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(waypoint.x, waypoint.y, 0.)),
                ..default()
            },
        ));
    })
}
