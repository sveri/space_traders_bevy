use bevy::{prelude::*, sprite::MaterialMesh2dBundle, text::TextStyle};

use crate::{
    controls,
    ship::Ship,
    st_client::{self, Waypoint},
};

#[derive(Resource)]
pub(crate) struct PlanetUpdateTimer(pub Timer);

#[derive(Resource)]
pub(crate) struct ShipUpdateTimer(pub Timer);

#[derive(Component)]
struct ShipRepresentation;

#[derive(Component)]
struct WaypointRepresentation;

pub(crate) fn show_ships(mut commands: Commands, ships: Query<&Ship>) {
    for ship in ships.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(ship.get_display_size().0, ship.get_display_size().1)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(ship.nav.route.departure.x, ship.nav.route.departure.y, 0.)),
                ..default()
            },
            ShipRepresentation,
        ));
    }
}

pub(crate) fn show_waypoints(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Waypoint>,
) {
    for waypoint in query.iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(waypoint.get_display_size()).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(waypoint.x, waypoint.y, 0.)),
                ..default()
            },
            WaypointRepresentation,
        ));
    }
}
