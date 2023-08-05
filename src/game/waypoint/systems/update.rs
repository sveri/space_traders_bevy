use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{st_client::Waypoint, game::waypoint::components::WaypointRepresentation};


pub(crate) fn show_waypoints(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Waypoint>,
) {
    for waypoint in query.iter() {
        // commands.spawn((
        //     MaterialMesh2dBundle {
        //         mesh: meshes.add(shape::Circle::new(waypoint.get_display_size()).into()).into(),
        //         material: materials.add(ColorMaterial::from(Color::PURPLE)),
        //         transform: Transform::from_translation(Vec3::new(waypoint.x, waypoint.y, 0.)),
        //         ..default()
        //     },
        //     WaypointRepresentation,
        // ));
    }
}